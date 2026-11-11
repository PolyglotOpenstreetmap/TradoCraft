import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../app/api.dart';
import 'draft.dart';

/// The revision editor's state: the revision, its definition being edited, the problems the
/// server reported when it was last saved, and what is selected on each tab.
class EditorState {
  const EditorState({
    this.revision,
    this.draft,
    this.version = 0,
    this.loading = true,
    this.loadError,
    this.dirty = false,
    this.busy = false,
    this.errors = const [],
    this.tab = EditorTab.fields,
    this.fieldScope = const ScopeSel.project(),
    this.fieldIndex,
    this.computedScope = const ScopeSel.project(),
    this.computedIndex,
    this.tableIndex,
    this.tableRow,
    this.rulePath,
    this.validationIndex,
    this.focusProperty,
    this.navigation = 0,
  });

  final Revision? revision;
  final DefinitionDraft? draft;

  /// Bumped on every edit so that widgets rebuild (the draft is changed in place).
  final int version;
  final bool loading;
  final String? loadError;
  final bool dirty;
  final bool busy;

  /// Problems of the saved definition, each with its JSON path.
  final List<DefinitionError> errors;
  final EditorTab tab;
  final ScopeSel fieldScope;
  final int? fieldIndex;
  final ScopeSel computedScope;
  final int? computedIndex;
  final int? tableIndex;
  final int? tableRow;
  final List<int>? rulePath;
  final int? validationIndex;

  /// The property an error navigation pointed at (highlighted in the editor).
  final String? focusProperty;

  /// Bumped on every navigation to an error.
  final int navigation;

  bool get readOnly => revision?.status != 'draft';

  /// Problems at [path] or below it.
  List<DefinitionError> errorsAt(String path) => [
    for (final e in errors)
      if (e.path == path ||
          e.path.startsWith('$path.') ||
          e.path.startsWith('$path['))
        e,
  ];

  /// Problems exactly at [path] with a character range, for a formula field.
  List<({int start, int end, String message})> formulaProblems(String path) => [
    for (final e in errors)
      if (e.path == path)
        (start: e.start ?? 0, end: e.end ?? 0, message: e.message),
  ];

  EditorState copyWith({
    Revision? revision,
    DefinitionDraft? draft,
    int? version,
    bool? loading,
    Object? loadError = _keep,
    bool? dirty,
    bool? busy,
    List<DefinitionError>? errors,
    EditorTab? tab,
    ScopeSel? fieldScope,
    Object? fieldIndex = _keep,
    ScopeSel? computedScope,
    Object? computedIndex = _keep,
    Object? tableIndex = _keep,
    Object? tableRow = _keep,
    Object? rulePath = _keep,
    Object? validationIndex = _keep,
    Object? focusProperty = _keep,
    int? navigation,
  }) => EditorState(
    revision: revision ?? this.revision,
    draft: draft ?? this.draft,
    version: version ?? this.version,
    loading: loading ?? this.loading,
    loadError: loadError == _keep ? this.loadError : loadError as String?,
    dirty: dirty ?? this.dirty,
    busy: busy ?? this.busy,
    errors: errors ?? this.errors,
    tab: tab ?? this.tab,
    fieldScope: fieldScope ?? this.fieldScope,
    fieldIndex: fieldIndex == _keep ? this.fieldIndex : fieldIndex as int?,
    computedScope: computedScope ?? this.computedScope,
    computedIndex: computedIndex == _keep
        ? this.computedIndex
        : computedIndex as int?,
    tableIndex: tableIndex == _keep ? this.tableIndex : tableIndex as int?,
    tableRow: tableRow == _keep ? this.tableRow : tableRow as int?,
    rulePath: rulePath == _keep ? this.rulePath : rulePath as List<int>?,
    validationIndex: validationIndex == _keep
        ? this.validationIndex
        : validationIndex as int?,
    focusProperty: focusProperty == _keep
        ? this.focusProperty
        : focusProperty as String?,
    navigation: navigation ?? this.navigation,
  );
}

const _keep = Object();

/// Loads, edits, saves and activates one revision of a configurator.
class RevisionEditorNotifier extends Notifier<EditorState> {
  RevisionEditorNotifier(this.id);

  final String id;

  TradocraftApi get _api => ref.read(apiProvider);

  @override
  EditorState build() {
    Future.microtask(load);
    return const EditorState();
  }

  Future<void> load() async {
    try {
      final r = await _api.configurator.getRevision(id: id);
      if (!ref.mounted) return;
      _loaded(r, keepSelection: false);
    } catch (e) {
      if (ref.mounted) {
        state = state.copyWith(loading: false, loadError: errorMessage(e));
      }
    }
  }

  void _loaded(Revision r, {required bool keepSelection}) {
    final base = keepSelection ? state : const EditorState();
    state = base.copyWith(
      revision: r,
      draft: DefinitionDraft(r.definition),
      version: state.version + 1,
      loading: false,
      loadError: null,
      dirty: false,
      errors: r.errors,
    );
  }

  /// Changes the draft in place (ignored on a revision that is not a draft).
  void edit(void Function(DefinitionDraft d) change) {
    final d = state.draft;
    if (d == null || state.readOnly) return;
    change(d);
    state = state.copyWith(version: state.version + 1, dirty: true);
  }

  /// Replaces the whole definition (JSON import, raw JSON editing).
  void replaceDefinition(Object? definition) {
    if (state.readOnly) return;
    state = state.copyWith(
      draft: DefinitionDraft(definition),
      version: state.version + 1,
      dirty: true,
      fieldIndex: null,
      computedIndex: null,
      tableIndex: null,
      tableRow: null,
      rulePath: null,
      validationIndex: null,
    );
  }

  void setTab(EditorTab t) => state = state.copyWith(tab: t);

  void selectField(ScopeSel scope, int? index) => state = state.copyWith(
    fieldScope: scope,
    fieldIndex: index,
    focusProperty: null,
  );

  void selectComputed(ScopeSel scope, int? index) => state = state.copyWith(
    computedScope: scope,
    computedIndex: index,
    focusProperty: null,
  );

  void selectTable(int? index, {int? row}) => state = state.copyWith(
    tableIndex: index,
    tableRow: row,
    focusProperty: null,
  );

  void selectRule(List<int>? path) =>
      state = state.copyWith(rulePath: path, focusProperty: null);

  void selectValidation(int? index) =>
      state = state.copyWith(validationIndex: index, focusProperty: null);

  /// Shows the element a JSON path points at (a server error).
  void goTo(String path) {
    final loc = locate(path);
    var s = state.copyWith(
      tab: loc.tab,
      focusProperty: loc.property,
      navigation: state.navigation + 1,
    );
    switch (loc.tab) {
      case EditorTab.fields:
        s = s.copyWith(fieldScope: loc.scope, fieldIndex: loc.index);
      case EditorTab.computed:
        s = s.copyWith(computedScope: loc.scope, computedIndex: loc.index);
      case EditorTab.tables:
        s = s.copyWith(tableIndex: loc.index, tableRow: loc.row);
      case EditorTab.outputs:
        s = s.copyWith(rulePath: loc.rule);
      case EditorTab.validations:
        s = s.copyWith(validationIndex: loc.index);
      default:
        break;
    }
    state = s;
  }

  /// Saves the whole definition (PUT); the server answers with its problems. Returns them.
  Future<List<DefinitionError>> save() async {
    final d = state.draft;
    final r = state.revision;
    if (d == null || r == null || state.readOnly) return state.errors;
    final sentVersion = state.version;
    state = state.copyWith(busy: true);
    try {
      final saved = await _api.configurator.updateRevision(
        id: id,
        body: RevisionUpdate(definition: d.toJson()),
      );
      if (!ref.mounted) return saved.errors;
      state = state.copyWith(
        revision: saved,
        errors: saved.errors,
        // Edits made while saving stay unsaved.
        dirty: state.version != sentVersion,
      );
      return saved.errors;
    } finally {
      if (ref.mounted) state = state.copyWith(busy: false);
    }
  }

  /// Saves when needed, then activates. Returns null on success, else the server's message (the
  /// problems are then shown with their paths).
  Future<String?> activate() async {
    if (state.readOnly) return null;
    if (state.dirty) await save();
    if (state.errors.isNotEmpty) return null;
    state = state.copyWith(busy: true);
    try {
      final r = await _api.configurator.activateRevision(id: id);
      if (ref.mounted) _loaded(r, keepSelection: true);
      return null;
    } on DioException catch (e) {
      // Refresh the problems the server sees.
      final r = await _api.configurator.getRevision(id: id);
      if (ref.mounted) {
        state = state.copyWith(revision: r, errors: r.errors);
      }
      return errorMessage(e);
    } finally {
      if (ref.mounted) state = state.copyWith(busy: false);
    }
  }
}

final revisionEditorProvider = NotifierProvider.autoDispose
    .family<RevisionEditorNotifier, EditorState, String>(
      RevisionEditorNotifier.new,
    );
