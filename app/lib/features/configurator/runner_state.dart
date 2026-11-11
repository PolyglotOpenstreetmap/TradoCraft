import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app/api.dart';
import 'definition.dart';

typedef InputRow = Map<String, Object?>;

/// What the form and the item grid may change. [RunnerNotifier] implements it; tests use a
/// simple in-memory version.
abstract interface class RunnerActions {
  void setValue(FieldRef ref, Object? value);
  void pin(FieldRef ref, Object? value);
  void unpin(FieldRef ref);
  void setDisplayUnit(String key, String unit);
  void addRow(String table, [InputRow initial]);
  void duplicateRow(String table, int row);
  void deleteRow(String table, int row);
  void bulkSet(String table, String name, Object? value, {Iterable<int>? rows});
}

/// Where a value lives: a project value, a value of one group, or a value of one item row.
class FieldRef {
  const FieldRef.project(this.name) : group = null, table = null, row = null;
  const FieldRef.group(String this.group, this.name) : table = null, row = null;
  const FieldRef.item(String this.table, int this.row, this.name)
    : group = null;

  final String name;
  final String? group;
  final String? table;

  /// 0-based row of [table].
  final int? row;

  bool get isProject => group == null && table == null;

  /// A stable key, also used to focus the field from the issues panel.
  String get key => table != null
      ? 'item/$table/$row/$name'
      : group != null
      ? 'group/$group/$name'
      : 'project/$name';

  /// The value an issue points at (`scope` as the engine reports it plus its `field`).
  static FieldRef? ofIssue(Issue issue) {
    final field = issue.field;
    if (field == null) return null;
    if (issue.scope == 'project') return FieldRef.project(field);
    final m = RegExp(r'^([A-Za-z_][A-Za-z0-9_]*)\[(.*)\]$')
        .firstMatch(issue.scope);
    if (m == null) return null;
    if (m.group(1) == 'groups') return FieldRef.group(m.group(2)!, field);
    final row = int.tryParse(m.group(2)!);
    return row == null ? null : FieldRef.item(m.group(1)!, row, field);
  }

  bool matchesPin(Pin p) =>
      p.name == name &&
      p.table == table &&
      p.row == row &&
      (table != null || p.group == group);

  Pin pin(Object? value, Object? calculated) => Pin(
    name: name,
    value: value,
    calculated: calculated,
    table: table,
    row: row,
    group: group,
  );

  @override
  bool operator ==(Object other) => other is FieldRef && other.key == key;

  @override
  int get hashCode => key.hashCode;

  @override
  String toString() => key;
}

Map<String, Object?> _copyMap(Object? v) => v is Map
    ? {for (final e in v.entries) '${e.key}': e.value}
    : <String, Object?>{};

/// What the user entered, in the shape `POST /configurator/revisions/{id}/evaluate` takes:
/// `{project: {name: v}, groups: {group: {name: v}}, items: {table: [{name: v}]}, pins: [...]}`.
/// Immutable; every change returns a copy. A null value removes the entry so the field's default
/// applies again.
class RunnerInputs {
  const RunnerInputs({
    this.project = const {},
    this.groups = const {},
    this.items = const {},
    this.pins = const [],
  });

  factory RunnerInputs.fromApi(Inputs? i) {
    if (i == null) return const RunnerInputs();
    final groups = <String, Map<String, Object?>>{};
    if (i.groups is Map) {
      for (final e in (i.groups as Map).entries) {
        groups['${e.key}'] = _copyMap(e.value);
      }
    }
    final items = <String, List<InputRow>>{};
    if (i.items is Map) {
      for (final e in (i.items as Map).entries) {
        items['${e.key}'] = [
          for (final r in (e.value as List? ?? const [])) _copyMap(r),
        ];
      }
    }
    return RunnerInputs(
      project: _copyMap(i.project),
      groups: groups,
      items: items,
      pins: [...?i.pins],
    );
  }

  final Map<String, Object?> project;
  final Map<String, Map<String, Object?>> groups;
  final Map<String, List<InputRow>> items;
  final List<Pin> pins;

  bool get isEmpty =>
      project.isEmpty &&
      groups.values.every((g) => g.isEmpty) &&
      items.values.every((rows) => rows.isEmpty) &&
      pins.isEmpty;

  Inputs toApi() =>
      Inputs(project: project, groups: groups, items: items, pins: pins);

  Map<String, Object?> toJson() => {
    'project': project,
    'groups': groups,
    'items': items,
    'pins': [for (final p in pins) p.toJson()],
  };

  List<InputRow> rows(String table) => items[table] ?? const [];

  Object? valueOf(FieldRef ref) {
    if (ref.table != null) {
      final r = rows(ref.table!);
      return ref.row! < r.length ? r[ref.row!][ref.name] : null;
    }
    if (ref.group != null) return groups[ref.group]?[ref.name];
    return project[ref.name];
  }

  Pin? pinFor(FieldRef ref) {
    for (final p in pins) {
      if (ref.matchesPin(p)) return p;
    }
    return null;
  }

  RunnerInputs _copy({
    Map<String, Object?>? project,
    Map<String, Map<String, Object?>>? groups,
    Map<String, List<InputRow>>? items,
    List<Pin>? pins,
  }) => RunnerInputs(
    project: project ?? this.project,
    groups: groups ?? this.groups,
    items: items ?? this.items,
    pins: pins ?? this.pins,
  );

  static Map<String, Object?> _put(
    Map<String, Object?> m,
    String name,
    Object? value,
  ) {
    final out = {...m};
    if (value == null || (value is String && value.isEmpty)) {
      out.remove(name);
    } else {
      out[name] = value;
    }
    return out;
  }

  /// Sets (or with null clears) the input at [ref].
  RunnerInputs set(FieldRef ref, Object? value) {
    if (ref.table != null) {
      final rows = [...this.rows(ref.table!)];
      if (ref.row! >= rows.length) return this;
      rows[ref.row!] = _put(rows[ref.row!], ref.name, value);
      return _copy(items: {...items, ref.table!: rows});
    }
    if (ref.group != null) {
      return _copy(
        groups: {
          ...groups,
          ref.group!: _put(groups[ref.group] ?? const {}, ref.name, value),
        },
      );
    }
    return _copy(project: _put(project, ref.name, value));
  }

  /// Appends a row, optionally with initial values (e.g. its group).
  RunnerInputs addRow(String table, [InputRow initial = const {}]) {
    final row = <String, Object?>{};
    for (final e in initial.entries) {
      if (e.value != null) row[e.key] = e.value;
    }
    return _copy(
      items: {
        ...items,
        table: [...rows(table), row],
      },
    );
  }

  /// Inserts a copy of [row] after it, with copies of its pins.
  RunnerInputs duplicateRow(String table, int row) {
    final list = [...rows(table)];
    if (row < 0 || row >= list.length) return this;
    list.insert(row + 1, {...list[row]});
    final shifted = <Pin>[];
    for (final p in pins) {
      if (p.table != table || p.row == null) {
        shifted.add(p);
      } else if (p.row! > row) {
        shifted.add(_withRow(p, p.row! + 1));
      } else if (p.row == row) {
        shifted
          ..add(p)
          ..add(_withRow(p, row + 1));
      } else {
        shifted.add(p);
      }
    }
    return _copy(items: {...items, table: list}, pins: shifted);
  }

  /// Removes [row]; its pins go, pins of later rows move up.
  RunnerInputs deleteRow(String table, int row) {
    final list = [...rows(table)];
    if (row < 0 || row >= list.length) return this;
    list.removeAt(row);
    final shifted = <Pin>[
      for (final p in pins)
        if (p.table != table || p.row == null || p.row! < row)
          p
        else if (p.row! > row)
          _withRow(p, p.row! - 1),
    ];
    return _copy(items: {...items, table: list}, pins: shifted);
  }

  /// Sets one column of a table at once: all rows, or the given ones.
  RunnerInputs bulkSet(
    String table,
    String name,
    Object? value, {
    Iterable<int>? rows,
  }) {
    final list = [...this.rows(table)];
    final only = rows?.toSet();
    for (var i = 0; i < list.length; i++) {
      if (only == null || only.contains(i)) {
        list[i] = _put(list[i], name, value);
      }
    }
    return _copy(items: {...items, table: list});
  }

  RunnerInputs pin(Pin pin) => _copy(
    pins: [
      for (final p in pins)
        if (!_samePinTarget(p, pin)) p,
      pin,
    ],
  );

  RunnerInputs unpin(FieldRef ref) => _copy(
    pins: [
      for (final p in pins)
        if (!ref.matchesPin(p)) p,
    ],
  );

  static bool _samePinTarget(Pin a, Pin b) =>
      a.name == b.name &&
      a.table == b.table &&
      a.row == b.row &&
      a.group == b.group;

  static Pin _withRow(Pin p, int row) => Pin(
    name: p.name,
    value: p.value,
    calculated: p.calculated,
    table: p.table,
    row: row,
    group: p.group,
  );
}

/// Looks up values of an [Evaluation] by place.
class EvalIndex {
  EvalIndex(this.evaluation) {
    final e = evaluation;
    if (e == null) return;
    for (final v in e.project) {
      _values[FieldRef.project(v.name).key] = v;
    }
    for (final g in e.groups) {
      for (final v in g.values) {
        _values[FieldRef.group(g.name, v.name).key] = v;
      }
    }
    for (final t in e.items) {
      for (final r in t.rows) {
        _rows['${t.table}/${r.index - 1}'] = r;
        for (final v in r.values) {
          _values[FieldRef.item(t.table, r.index - 1, v.name).key] = v;
        }
      }
    }
    for (final i in e.issues) {
      final ref = FieldRef.ofIssue(i);
      if (ref != null) (_issues[ref.key] ??= []).add(i);
    }
  }

  final Evaluation? evaluation;
  final _values = <String, ValueResult>{};
  final _rows = <String, ItemResult>{};
  final _issues = <String, List<Issue>>{};

  ValueResult? operator [](FieldRef ref) => _values[ref.key];
  ItemResult? row(String table, int row) => _rows['$table/$row'];
  List<Issue> issuesAt(FieldRef ref) => _issues[ref.key] ?? const [];
  List<String> get groupNames => [
    for (final g in evaluation?.groups ?? const <GroupResult>[]) g.name,
  ];
  bool get hasErrors =>
      evaluation?.issues.any((i) => i.severity == IssueSeverity.error) ?? false;
}

/// One configuration being edited.
class RunnerState {
  const RunnerState({
    this.configuration,
    this.revision,
    this.definition,
    this.inputs = const RunnerInputs(),
    this.evaluation,
    this.loading = true,
    this.evaluating = false,
    this.busy = false,
    this.dirty = false,
    this.loadError,
    this.evalError,
    this.customerId,
    this.customerName,
    this.name = '',
    this.displayUnits = const {},
    this.pricing,
    this.pricedPositions = const {},
    this.pricingError,
  });

  final Configuration? configuration;
  final Revision? revision;
  final CfgDefinition? definition;
  final RunnerInputs inputs;

  /// The last successful evaluation (kept while a newer one is running or failed).
  final Evaluation? evaluation;
  final bool loading;
  final bool evaluating;

  /// Saving, quoting or upgrading.
  final bool busy;

  /// Inputs, name or customer changed since the last save.
  final bool dirty;
  final String? loadError;
  final String? evalError;
  final String? customerId;
  final String? customerName;
  final String name;

  /// Display unit chosen per field (key: `name` or `table.name`).
  final Map<String, String> displayUnits;

  /// Prices of the generated lines for the chosen customer.
  final PricingPreview? pricing;

  /// Line path (`0.1`) → index in [pricing]'s lines.
  final Map<String, int> pricedPositions;
  final String? pricingError;

  EvalIndex get index => EvalIndex(evaluation);

  RunnerState copyWith({
    Configuration? configuration,
    Revision? revision,
    CfgDefinition? definition,
    RunnerInputs? inputs,
    Evaluation? evaluation,
    bool? loading,
    bool? evaluating,
    bool? busy,
    bool? dirty,
    String? loadError,
    Object? evalError = _keep,
    Object? customerId = _keep,
    Object? customerName = _keep,
    String? name,
    Map<String, String>? displayUnits,
    Object? pricing = _keep,
    Map<String, int>? pricedPositions,
    Object? pricingError = _keep,
  }) => RunnerState(
    configuration: configuration ?? this.configuration,
    revision: revision ?? this.revision,
    definition: definition ?? this.definition,
    inputs: inputs ?? this.inputs,
    evaluation: evaluation ?? this.evaluation,
    loading: loading ?? this.loading,
    evaluating: evaluating ?? this.evaluating,
    busy: busy ?? this.busy,
    dirty: dirty ?? this.dirty,
    loadError: loadError ?? this.loadError,
    evalError: evalError == _keep ? this.evalError : evalError as String?,
    customerId: customerId == _keep ? this.customerId : customerId as String?,
    customerName: customerName == _keep
        ? this.customerName
        : customerName as String?,
    name: name ?? this.name,
    displayUnits: displayUnits ?? this.displayUnits,
    pricing: pricing == _keep ? this.pricing : pricing as PricingPreview?,
    pricedPositions: pricedPositions ?? this.pricedPositions,
    pricingError: pricingError == _keep
        ? this.pricingError
        : pricingError as String?,
  );
}

const _keep = Object();

/// The path of a line in the tree, e.g. `0.2.1`.
String linePath(List<int> path) => path.join('.');

/// Line at [path], or null.
Line? lineAt(List<Line> lines, List<int> path) {
  Line? found;
  var level = lines;
  for (final i in path) {
    if (i < 0 || i >= level.length) return null;
    found = level[i];
    level = found.children;
  }
  return found;
}

/// State and actions of the configuration runner. Every input change re-evaluates on the server
/// after a short pause; the last good evaluation stays on screen meanwhile.
class RunnerNotifier extends Notifier<RunnerState> implements RunnerActions {
  RunnerNotifier(this.id);

  final String id;

  /// Pause after the last change before evaluating (shorter in tests).
  static Duration debounce = const Duration(milliseconds: 300);

  Timer? _timer;
  int _evalSeq = 0;
  int _priceSeq = 0;

  /// Language of the product names of the lines (the screen sets the app's language).
  String? language;

  TradocraftApi get _api => ref.read(apiProvider);

  @override
  RunnerState build() {
    ref.onDispose(() => _timer?.cancel());
    Future.microtask(load);
    return const RunnerState();
  }

  Future<void> load() async {
    try {
      final config = await _api.configurator.getConfiguration(id: id);
      final revision = await _api.configurator.getRevision(
        id: config.revisionId,
      );
      if (!ref.mounted) return;
      final def = CfgDefinition(revision.definition);
      var inputs = RunnerInputs.fromApi(config.inputs);
      var dirty = false;
      // A new configuration (never saved since it was created): start with the rows a table
      // needs. A saved configuration keeps its (possibly empty) tables.
      if (inputs.isEmpty && !config.updatedAt.isAfter(config.createdAt)) {
        for (final t in def.itemTables) {
          final n = t.initialRows;
          for (var i = 0; i < n; i++) {
            inputs = inputs.addRow(t.name);
          }
          dirty = dirty || n > 0;
        }
      }
      Evaluation? stored;
      final result = config.result;
      if (result is Map && !dirty) {
        try {
          stored = Evaluation.fromJson(result.cast<String, Object?>());
        } catch (_) {
          stored = null;
        }
      }
      state = RunnerState(
        configuration: config,
        revision: revision,
        definition: def,
        inputs: inputs,
        evaluation: stored,
        loading: false,
        dirty: dirty,
        customerId: config.customerId,
        customerName: config.customerName,
        name: config.name,
        displayUnits: state.displayUnits,
      );
      await evaluate();
    } catch (e) {
      if (ref.mounted) {
        state = state.copyWith(loading: false, loadError: errorMessage(e));
      }
    }
  }

  void _changed(RunnerInputs inputs) {
    state = state.copyWith(inputs: inputs, dirty: true);
    _timer?.cancel();
    _timer = Timer(debounce, evaluate);
  }

  /// Sets (null clears) the input at [ref].
  @override
  void setValue(FieldRef ref, Object? value) =>
      _changed(state.inputs.set(ref, value));

  @override
  void addRow(String table, [InputRow initial = const {}]) =>
      _changed(state.inputs.addRow(table, initial));

  @override
  void duplicateRow(String table, int row) =>
      _changed(state.inputs.duplicateRow(table, row));

  @override
  void deleteRow(String table, int row) =>
      _changed(state.inputs.deleteRow(table, row));

  @override
  void bulkSet(
    String table,
    String name,
    Object? value, {
    Iterable<int>? rows,
  }) => _changed(state.inputs.bulkSet(table, name, value, rows: rows));

  /// Pins [value] on an overridable computed value, remembering what it calculates now.
  @override
  void pin(FieldRef ref, Object? value) {
    final existing = state.inputs.pinFor(ref);
    final current = state.index[ref];
    final shownPinned =
        current != null &&
        (current.source == ValueSource.pinned ||
            current.source == ValueSource.stale);
    final calculated = shownPinned
        ? current.calculated
        : existing != null
        ? existing.calculated
        : current?.value;
    if (calculated != null && sameValue(value, calculated)) {
      // Choosing what the formula gives is the same as no pin.
      if (existing != null) _changed(state.inputs.unpin(ref));
      return;
    }
    if (existing != null && sameValue(existing.value, value)) return;
    _changed(state.inputs.pin(ref.pin(value, calculated)));
  }

  /// Back to the calculated value.
  @override
  void unpin(FieldRef ref) => _changed(state.inputs.unpin(ref));

  void setName(String name) {
    if (name == state.name) return;
    state = state.copyWith(name: name, dirty: true);
  }

  void setCustomer(String? id, String? name) {
    state = state.copyWith(customerId: id, customerName: name, dirty: true);
    _price();
  }

  @override
  void setDisplayUnit(String key, String unit) =>
      state = state.copyWith(displayUnits: {...state.displayUnits, key: unit});

  /// Evaluates the current inputs now.
  Future<void> evaluate() async {
    _timer?.cancel();
    final revision = state.revision;
    if (revision == null) return;
    final seq = ++_evalSeq;
    state = state.copyWith(evaluating: true);
    try {
      final e = await _api.configurator.evaluateRevision(
        id: revision.id,
        body: state.inputs.toApi(),
        language: language,
      );
      if (!ref.mounted || seq != _evalSeq) return;
      state = state.copyWith(evaluation: e, evaluating: false, evalError: null);
      await _price();
    } catch (e) {
      if (!ref.mounted || seq != _evalSeq) return;
      state = state.copyWith(evaluating: false, evalError: errorMessage(e));
    }
  }

  /// Prices the line tree with `POST /pricing/preview` for the chosen customer. Lines whose code
  /// is not a product (and their options) are left out; the issues panel reports them.
  Future<void> _price() async {
    // Taken first, so that clearing the prices also discards a preview still under way.
    final seq = ++_priceSeq;
    final customer = state.customerId;
    final lines = state.evaluation?.lines ?? const <Line>[];
    if (customer == null || lines.isEmpty) {
      state = state.copyWith(
        pricing: null,
        pricedPositions: const {},
        pricingError: null,
      );
      return;
    }
    try {
      final inputs = <QuotationLineInput>[];
      final positions = <String, int>{};
      void add(List<Line> ls, List<int> path, int? parent) {
        for (final (i, l) in ls.indexed) {
          final productId = l.productId;
          if (productId == null) continue;
          inputs.add(
            QuotationLineInput(
              productId: productId,
              quantity: l.quantity,
              parentPosition: parent,
              section: l.section,
              comment: l.comment,
              description: l.description,
              percentOfParent: parent == null ? null : l.percentOfParent,
              percentBase: l.percentBase == null
                  ? null
                  : PercentBase.fromJson(l.percentBase!),
            ),
          );
          final me = inputs.length;
          positions[linePath([...path, i])] = me - 1;
          add(l.children, [...path, i], me);
        }
      }

      add(lines, const [], null);
      if (inputs.isEmpty) {
        if (seq == _priceSeq && ref.mounted) {
          state = state.copyWith(pricing: null, pricedPositions: const {});
        }
        return;
      }
      final preview = await _api.sales.preview(
        body: PricingPreviewInput(customerId: customer, lines: inputs),
      );
      if (!ref.mounted || seq != _priceSeq) return;
      state = state.copyWith(
        pricing: preview,
        pricedPositions: positions,
        pricingError: null,
      );
    } catch (e) {
      if (!ref.mounted || seq != _priceSeq) return;
      state = state.copyWith(pricing: null, pricingError: errorMessage(e));
    }
  }

  ConfigurationInput _body({String? revisionId}) => ConfigurationInput(
    customerId: state.customerId,
    name: state.name,
    inputs: state.inputs.toApi(),
    revisionId: revisionId,
  );

  /// Whether name, customer and inputs are still what was sent (edits made while a request was
  /// under way stay unsaved).
  bool Function() _unchangedSince() {
    final inputs = state.inputs, name = state.name, customer = state.customerId;
    return () =>
        identical(state.inputs, inputs) &&
        state.name == name &&
        state.customerId == customer;
  }

  /// Saves name, customer and inputs.
  Future<void> save() async {
    final unchanged = _unchangedSince();
    state = state.copyWith(busy: true);
    try {
      final c = await _api.configurator.updateConfiguration(
        id: id,
        body: _body(),
      );
      if (!ref.mounted) return;
      state = state.copyWith(configuration: c, dirty: !unchanged());
    } finally {
      if (ref.mounted) state = state.copyWith(busy: false);
    }
  }

  /// Saves when needed, then creates or updates the quotation.
  Future<ConfiguredImportResult> quote() async {
    if (state.dirty) await save();
    // Once more for edits made during that save, so the quotation has what is on screen.
    if (state.dirty) await save();
    state = state.copyWith(busy: true);
    try {
      final r = await _api.configurator.quoteConfiguration(
        id: id,
        body: const QuoteInput(),
      );
      final c = await _api.configurator.getConfiguration(id: id);
      if (ref.mounted) state = state.copyWith(configuration: c);
      return r;
    } finally {
      if (ref.mounted) state = state.copyWith(busy: false);
    }
  }

  /// Moves the configuration to the model's newer active revision (saving the inputs).
  Future<void> upgrade() async {
    final newer = state.configuration?.newerRevisionId;
    if (newer == null) return;
    final unchanged = _unchangedSince();
    state = state.copyWith(busy: true);
    try {
      final c = await _api.configurator.updateConfiguration(
        id: id,
        body: _body(revisionId: newer),
      );
      final r = await _api.configurator.getRevision(id: c.revisionId);
      if (!ref.mounted) return;
      state = state.copyWith(
        configuration: c,
        revision: r,
        definition: CfgDefinition(r.definition),
        dirty: !unchanged(),
      );
      await evaluate();
    } finally {
      if (ref.mounted) state = state.copyWith(busy: false);
    }
  }
}

final runnerProvider = NotifierProvider.autoDispose
    .family<RunnerNotifier, RunnerState, String>(RunnerNotifier.new);
