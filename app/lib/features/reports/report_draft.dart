import '../../app/api.dart';

/// Looking things up in the catalog the server sent (the objects the person may read).
extension CatalogLookup on ReportCatalog {
  /// The object called [name], or null.
  ReportCatalogObject? objectNamed(String name) {
    for (final o in objects) {
      if (o.name == name) return o;
    }
    return null;
  }

  /// The field with the qualified name `object.field`, or null.
  ReportCatalogField? field(String qualified) {
    for (final o in objects) {
      for (final f in o.fields) {
        if (f.name == qualified) return f;
      }
    }
    return null;
  }

  /// Objects that can be added next to [included]: related to one of them, not in yet.
  List<ReportCatalogObject> joinable(List<String> included) => [
    for (final o in objects)
      if (!included.contains(o.name) &&
          links.any(
            (l) =>
                (l.a == o.name && included.contains(l.b)) ||
                (l.b == o.name && included.contains(l.a)),
          ))
        o,
  ];

  /// Fields of the objects in the report (the base object first).
  List<ReportCatalogField> fieldsOf(List<String> included) => [
    for (final name in included) ...?objectNamed(name)?.fields,
  ];
}

/// Operators that make sense for a field type, in the order the form lists them.
List<ReportOp> operatorsFor(ReportFieldType type) => switch (type) {
  ReportFieldType.text => [
    ReportOp.eq,
    ReportOp.ne,
    ReportOp.contains,
    ReportOp.startsWith,
    ReportOp.valueIn,
    ReportOp.isNull,
    ReportOp.notNull,
  ],
  ReportFieldType.number => [
    ReportOp.eq,
    ReportOp.ne,
    ReportOp.gt,
    ReportOp.gte,
    ReportOp.lt,
    ReportOp.lte,
    ReportOp.valueIn,
    ReportOp.isNull,
    ReportOp.notNull,
  ],
  ReportFieldType.date || ReportFieldType.datetime => [
    ReportOp.eq,
    ReportOp.ne,
    ReportOp.gt,
    ReportOp.gte,
    ReportOp.lt,
    ReportOp.lte,
    ReportOp.valueIn,
    ReportOp.isNull,
    ReportOp.notNull,
  ],
  ReportFieldType.valueBool => [
    ReportOp.eq,
    ReportOp.ne,
    ReportOp.isNull,
    ReportOp.notNull,
  ],
  _ => [ReportOp.eq],
};

/// Summaries a field supports. [grouped] drops "show the value": in a grouped report every
/// column is a summary.
List<ReportSummary?> summariesFor(
  ReportCatalogField field, {
  required bool grouped,
}) {
  final base = <ReportSummary?>[
    if (!grouped) null,
    ReportSummary.count,
    if (!field.countOnly && field.type == ReportFieldType.number) ...[
      ReportSummary.sum,
      ReportSummary.avg,
    ],
    if (!field.countOnly &&
        (field.type == ReportFieldType.number ||
            field.type == ReportFieldType.date ||
            field.type == ReportFieldType.datetime)) ...[
      ReportSummary.min,
      ReportSummary.max,
    ],
  ];
  return field.countOnly && !grouped ? [ReportSummary.count] : base;
}

/// An output column being edited: the field, how it is summarised and its custom heading.
class DraftColumn {
  DraftColumn(this.field, [this.summary, this.label]);
  String field;
  ReportSummary? summary;

  /// Custom heading, or null for the default derived from the field. Kept so editing a saved
  /// report does not lose it.
  String? label;
}

class DraftFilter {
  DraftFilter(this.field, this.op, [this.value = '']);
  String field;
  ReportOp op;

  /// What the person typed; a comma separated list for "is one of".
  String value;

  /// The relative date token (see [relativeTokens]) when a date filter is relative, else null.
  String? relative;

  /// What was typed for the `n` of `last_n_days` and `next_n_days`.
  String days = '30';

  /// Whether [relative] is a token that takes a number of days.
  bool get relativeNeedsDays => relativeTakesDays(relative);

  /// The number of days when it is a whole number from 1 to [maxRelativeDays], else null.
  int? get relativeDays {
    final n = int.tryParse(days.trim());
    return n != null && n >= 1 && n <= maxRelativeDays ? n : null;
  }

  /// A relative filter whose number of days the server would refuse.
  bool get relativeInvalid =>
      relative != null && relativeNeedsDays && relativeDays == null;
}

/// The most days a relative filter may span (the server's limit).
const maxRelativeDays = 3660;

/// Relative date tokens, in the order the form lists them.
const relativeTokens = [
  'today',
  'yesterday',
  'this_week',
  'last_week',
  'this_month',
  'last_month',
  'this_quarter',
  'last_quarter',
  'this_year',
  'last_year',
  'last_n_days',
  'next_n_days',
];

/// Whether a relative date [token] takes a number of days.
bool relativeTakesDays(String? token) =>
    token == 'last_n_days' || token == 'next_n_days';

/// Operators a relative date can go with (not in, is empty, is not empty).
bool relativeAllowedFor(ReportOp op) =>
    op != ReportOp.valueIn && op != ReportOp.isNull && op != ReportOp.notNull;

bool _isDateType(ReportFieldType? t) =>
    t == ReportFieldType.date || t == ReportFieldType.datetime;

/// A grouping being edited: the field and, for dates, the bucket.
class DraftGroup {
  DraftGroup(this.field, [this.bucket]);
  String field;
  ReportBucket? bucket;
}

/// A sort being edited: the output column key and its direction.
class DraftSort {
  DraftSort(this.column, {this.descending = false});
  String column;
  bool descending;
}

/// An output column of the report, as sort and chart name it.
typedef OutputKey = ({String key, String label});

/// The report being edited: a mutable copy of a definition plus its name and sharing.
class ReportDraft {
  ReportDraft({required this.object});

  String name = '';
  String description = '';
  bool shared = false;
  String object;
  final List<String> joins = [];
  final List<DraftColumn> columns = [];
  final List<DraftFilter> filters = [];
  ReportLogic logic = ReportLogic.and;
  final List<DraftGroup> groups = [];
  final List<DraftSort> sorts = [];
  ReportChartKind? chartKind;
  String? chartX;
  String? chartY;

  /// The base object followed by the joined ones.
  List<String> get included => [object, ...joins];

  /// Whether the report has at least one grouping.
  bool get grouped => groups.isNotEmpty;

  /// Whether a relative filter has a number of days the server would refuse.
  bool get hasInvalidRelative => filters.any((f) => f.relativeInvalid);

  /// A draft holding a copy of the saved report [r].
  factory ReportDraft.from(SavedReport r) {
    final d = r.definition;
    final draft = ReportDraft(object: d.object)
      ..name = r.name
      ..description = r.description
      ..shared = r.shared
      ..logic = d.filterLogic ?? ReportLogic.and
      ..chartKind = d.chart?.kind
      ..chartX = d.chart?.x
      ..chartY = d.chart?.y;
    draft.joins.addAll(d.joins ?? const []);
    for (final c in d.columns) {
      draft.columns.add(DraftColumn(c.field, c.summary, c.label));
    }
    for (final f in d.filters ?? const <ReportFilter>[]) {
      final filter = DraftFilter(f.field, f.op, _valueText(f.value));
      final v = f.value;
      if (v is Map && v['relative'] is String) {
        filter
          ..value = ''
          ..relative = v['relative'] as String;
        final n = v['n'];
        if (n != null) filter.days = n.toString();
      }
      draft.filters.add(filter);
    }
    for (final g in d.groupBy ?? const <ReportGroupBy>[]) {
      draft.groups.add(DraftGroup(g.field, g.bucket));
    }
    for (final s in d.sort ?? const <ReportSort>[]) {
      draft.sorts.add(DraftSort(s.column, descending: s.dir == ReportDir.desc));
    }
    return draft;
  }

  static String _valueText(dynamic v) {
    if (v == null) return '';
    if (v is List) return _joinList(v);
    return v.toString();
  }

  /// The output columns in the order the report shows them: groupings first.
  List<OutputKey> outputKeys(ReportCatalog catalog) {
    String labelOf(String field) => catalog.field(field)?.label ?? field;
    return [
      for (final g in groups)
        (
          key: g.bucket == null ? g.field : '${g.field}:${g.bucket!.json}',
          label: labelOf(g.field),
        ),
      for (final c in columns)
        (
          key: c.summary == null ? c.field : '${c.field}:${c.summary!.json}',
          label: labelOf(c.field),
        ),
    ];
  }

  /// Output keys that are summaries a chart can plot: counts, and any summary of a numeric
  /// field (not min or max of a date).
  List<OutputKey> summaryKeys(ReportCatalog catalog) => [
    for (final c in columns)
      if (c.summary == ReportSummary.count ||
          (c.summary != null &&
              catalog.field(c.field)?.type == ReportFieldType.number))
        (
          key: '${c.field}:${c.summary!.json}',
          label: catalog.field(c.field)?.label ?? c.field,
        ),
  ];

  /// Output keys that are groupings (what a chart can use as categories).
  List<OutputKey> groupKeys(ReportCatalog catalog) => [
    for (final g in groups)
      (
        key: g.bucket == null ? g.field : '${g.field}:${g.bucket!.json}',
        label: catalog.field(g.field)?.label ?? g.field,
      ),
  ];

  /// The definition the server runs and stores, built from this draft.
  ReportDefinition toDefinition(ReportCatalog catalog) {
    final keys = outputKeys(catalog).map((k) => k.key).toSet();
    final summaryKeySet = summaryKeys(catalog).map((k) => k.key).toSet();
    final groupKeySet = groupKeys(catalog).map((k) => k.key).toSet();
    final chart =
        grouped &&
            chartKind != null &&
            groupKeySet.contains(chartX) &&
            summaryKeySet.contains(chartY)
        ? ReportChart(kind: chartKind!, x: chartX!, y: chartY!)
        : null;
    return ReportDefinition(
      object: object,
      joins: joins.isEmpty ? null : List.of(joins),
      columns: [
        for (final c in columns)
          ReportColumn(field: c.field, label: c.label, summary: c.summary),
      ],
      filters: [
        for (final f in filters)
          ReportFilter(
            field: f.field,
            op: f.op,
            value: _draftFilterValue(catalog.field(f.field)?.type, f),
          ),
      ],
      filterLogic: logic,
      groupBy: [
        for (final g in groups) ReportGroupBy(field: g.field, bucket: g.bucket),
      ],
      sort: [
        for (final s in sorts)
          if (keys.contains(s.column))
            ReportSort(
              column: s.column,
              dir: s.descending ? ReportDir.desc : ReportDir.asc,
            ),
      ],
      chart: chart,
    );
  }
}

/// The value of [f] as sent: a relative date object or the typed value.
dynamic _draftFilterValue(ReportFieldType? type, DraftFilter f) {
  if (f.relative != null && _isDateType(type) && relativeAllowedFor(f.op)) {
    return {
      'relative': f.relative,
      if (f.relativeNeedsDays) 'n': f.relativeDays ?? f.days.trim(),
    };
  }
  return _filterValue(type, f.op, f.value);
}

/// The typed value for a filter, from what the person typed.
dynamic filterValue(ReportFieldType? type, ReportOp op, String text) =>
    _filterValue(type, op, text);

dynamic _filterValue(ReportFieldType? type, ReportOp op, String text) {
  if (op == ReportOp.isNull || op == ReportOp.notNull) return null;
  final t = text.trim();
  if (op == ReportOp.valueIn) return _splitList(t);
  switch (type) {
    case ReportFieldType.number:
      // The server accepts numbers written as text too, so "1,5" or "12.50" is not lost here.
      return num.tryParse(t) ?? t.replaceAll(',', '.');
    case ReportFieldType.valueBool:
      // An empty value shows as "Yes" in the builder, so it must be sent as true.
      return t.isEmpty || t == 'true';
    default:
      return t;
  }
}

/// Splits a typed list on commas; an item may be wrapped in double quotes (with `""` for a
/// quote) to hold commas. Empty items are dropped.
List<String> _splitList(String text) {
  final out = <String>[];
  final b = StringBuffer();
  var quoted = false;
  var wasQuoted = false;
  void flush() {
    final item = wasQuoted ? b.toString() : b.toString().trim();
    if (item.isNotEmpty) out.add(item);
    b.clear();
    wasQuoted = false;
  }

  for (var i = 0; i < text.length; i++) {
    final c = text[i];
    if (quoted) {
      if (c == '"') {
        if (i + 1 < text.length && text[i + 1] == '"') {
          b.write('"');
          i++;
        } else {
          quoted = false;
        }
      } else {
        b.write(c);
      }
    } else if (c == '"' && b.toString().trim().isEmpty) {
      b.clear();
      quoted = true;
      wasQuoted = true;
    } else if (c == ',') {
      flush();
    } else {
      b.write(c);
    }
  }
  flush();
  return out;
}

/// Writes a list the way [_splitList] reads it back: items with commas, quotes or edge
/// spaces are quoted.
String _joinList(List<dynamic> items) => [
  for (final v in items)
    if (v.toString().contains(RegExp(r'[,"]')) ||
        v.toString() != v.toString().trim())
      '"${v.toString().replaceAll('"', '""')}"'
    else
      v.toString(),
].join(', ');
