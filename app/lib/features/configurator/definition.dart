import 'package:decimal/decimal.dart';

/// Read-only view of a configurator definition (ADR 0034) as far as the runner needs it: the
/// fields and computed values per scope, their labels, units and layout. The server validates
/// and evaluates the definition; the app only renders it.

Map<String, Object?> _map(Object? v) =>
    v is Map ? v.cast<String, Object?>() : const <String, Object?>{};

List<Map<String, Object?>> _list(Object? v) =>
    v is List ? [for (final e in v) _map(e)] : const <Map<String, Object?>>[];

String? _str(Object? v) => v is String ? v : null;

String? _dec(Object? v) => switch (v) {
  num n => n.toString(),
  String s when s.isNotEmpty => s,
  _ => null,
};

/// Label in [lang] from a `translations` map, else [fallback].
String _translated(Object? translations, String lang, String fallback) {
  final t = _map(_map(translations)[lang]);
  final l = _str(t['label']);
  return (l != null && l.isNotEmpty) ? l : fallback;
}

String _help(Object? translations, String lang, String fallback) {
  final t = _map(_map(translations)[lang]);
  final h = _str(t['help']);
  return (h != null && h.isNotEmpty) ? h : fallback;
}

/// Field types of the definition.
enum CfgFieldType { number, integer, boolean, choice, text, product }

class CfgOption {
  const CfgOption(this.value, this.label, this.translations);
  final Object? value;
  final String label;
  final Object? translations;

  String labelIn(String lang) => _translated(
    translations,
    lang,
    label.isEmpty ? displayValue(value) : label,
  );
}

/// Common part of fields and computed values: what a cell or form row shows.
sealed class CfgEntry {
  CfgEntry(this.json);
  final Map<String, Object?> json;

  String get name => _str(json['name']) ?? '';
  String get rawLabel => _str(json['label']) ?? '';
  String labelIn(String lang) => _translated(
    json['translations'],
    lang,
    rawLabel.isEmpty ? name : rawLabel,
  );
  String helpIn(String lang) =>
      _help(json['translations'], lang, _str(json['help']) ?? '');
  String? get unit => _str(json['unit']);
  Map<String, Object?> get layout => _map(json['layout']);
  String? get section => _str(layout['section']);
  int get order => (layout['order'] as num?)?.toInt() ?? 0;

  /// The form column (1-based) the designer placed this entry in, if any.
  int? get column => (layout['column'] as num?)?.toInt();
  int get width => (layout['width'] as num?)?.toInt() ?? 1;
}

class CfgField extends CfgEntry {
  CfgField(super.json);

  CfgFieldType get type => switch (json['type']) {
    'number' => CfgFieldType.number,
    'integer' => CfgFieldType.integer,
    'boolean' => CfgFieldType.boolean,
    'choice' => CfgFieldType.choice,
    'product' => CfgFieldType.product,
    _ => CfgFieldType.text,
  };
  bool get required => json['required'] == true;
  Object? get defaultValue => json['default'];
  String? get dimension => _str(json['dimension']);
  List<String> get displayUnits => [
    for (final u in (json['display_units'] as List? ?? const [])) '$u',
  ];
  String? get min => _dec(json['min']);
  String? get max => _dec(json['max']);
  String? get step => _dec(json['step']);
  int? get maxLength => (json['max_length'] as num?)?.toInt();
  String? get family => _str(json['family']);
  List<CfgOption> get options => [
    for (final o in _list(json['options']))
      CfgOption(o['value'], _str(o['label']) ?? '', o['translations']),
  ];

  /// The units the user may type in: the field's unit first, then its display units.
  List<String> get inputUnits {
    final u = unit;
    if (u == null) return const [];
    return [
      u,
      for (final d in displayUnits)
        if (d != u && unitFactor(dimension, d) != null) d,
    ];
  }

  /// Label of a choice value (the option's label, else the value itself).
  String optionLabel(Object? value, String lang) {
    for (final o in options) {
      if (sameValue(o.value, value)) return o.labelIn(lang);
    }
    return displayValue(value);
  }
}

class CfgComputed extends CfgEntry {
  CfgComputed(super.json);

  bool get overridable => json['overridable'] == true;
  bool get show => json['show'] != false;
}

/// The fields and computed values of one scope.
class CfgScope {
  CfgScope(Map<String, Object?> json)
    : fields = [for (final f in _list(json['fields'])) CfgField(f)],
      computed = [for (final c in _list(json['computed'])) CfgComputed(c)];
  final List<CfgField> fields;
  final List<CfgComputed> computed;

  bool get isEmpty => fields.isEmpty && !computed.any((c) => c.show);

  CfgEntry? entry(String name) {
    for (final f in fields) {
      if (f.name == name) return f;
    }
    for (final c in computed) {
      if (c.name == name) return c;
    }
    return null;
  }
}

class CfgItemTable extends CfgScope {
  CfgItemTable(this.json) : super(json);
  final Map<String, Object?> json;

  String get name => _str(json['name']) ?? '';
  String labelIn(String lang) {
    final l = _str(json['label']) ?? '';
    return _translated(json['translations'], lang, l.isEmpty ? name : l);
  }

  String? get groupField => _str(json['group_field']);
  int? get minRows => (json['min_rows'] as num?)?.toInt();
  int? get maxRows => (json['max_rows'] as num?)?.toInt();

  /// Rows a new configuration starts with: `min_rows` (one by default), never more than
  /// `max_rows` (which may be zero) and at most 20.
  int get initialRows {
    final max = maxRows;
    var n = minRows ?? 1;
    if (max != null && n > max) n = max;
    return n.clamp(0, 20);
  }
}

class CfgSection {
  const CfgSection(this.name, this.label, this.translations);
  final String name;
  final String label;
  final Object? translations;
  String labelIn(String lang) =>
      _translated(translations, lang, label.isEmpty ? name : label);
}

class CfgDefinition {
  CfgDefinition(Object? raw) : json = _map(raw) {
    project = CfgScope(_map(json['project']));
    final g = _map(json['group']);
    group = CfgScope(g);
    groupLabel = _str(g['label']) ?? '';
    groupTranslations = g['translations'];
    itemTables = [for (final t in _list(json['item_tables'])) CfgItemTable(t)];
    sections = [
      for (final s in _list(json['sections']))
        CfgSection(
          _str(s['name']) ?? '',
          _str(s['label']) ?? '',
          s['translations'],
        ),
    ];
  }

  final Map<String, Object?> json;
  late final CfgScope project;
  late final CfgScope group;
  late final String groupLabel;
  late final Object? groupTranslations;
  late final List<CfgItemTable> itemTables;
  late final List<CfgSection> sections;

  String get name => _str(json['name']) ?? '';
  String get description => _str(json['description']) ?? '';
  String groupLabelIn(String lang) => _translated(
    groupTranslations,
    lang,
    groupLabel.isEmpty ? 'Group' : groupLabel,
  );

  CfgItemTable? table(String name) {
    for (final t in itemTables) {
      if (t.name == name) return t;
    }
    return null;
  }
}

/// [entries] sorted by `layout.order`, keeping the declaration order for equal orders.
List<CfgEntry> byLayoutOrder(List<CfgEntry> entries) {
  final indexed = entries.indexed.toList()
    ..sort((a, b) {
      final c = a.$2.order.compareTo(b.$2.order);
      return c != 0 ? c : a.$1.compareTo(b.$1);
    });
  return [for (final (_, e) in indexed) e];
}

// ---------------------------------------------------------------------------------------------
// Values and units
// ---------------------------------------------------------------------------------------------

/// Whether two JSON values mean the same (`3`, `3.0` and `'3'` are equal: the server sends
/// decimals as text). Text that is not written as a plain number, such as the code `'05'`, is
/// compared exactly, so `'05'` and `'5'` stay different.
bool sameValue(Object? a, Object? b) {
  if (a == b) return true;
  if (a == null || b == null) return false;
  final sa = '$a', sb = '$b';
  if ((a is String && !_plainNumber.hasMatch(sa)) ||
      (b is String && !_plainNumber.hasMatch(sb))) {
    return sa == sb;
  }
  final da = Decimal.tryParse(sa), db = Decimal.tryParse(sb);
  if (da != null && db != null) return da == db;
  return sa == sb;
}

/// A number as the server writes one: no leading zeros, plus sign or exponent.
final _plainNumber = RegExp(r'^-?(0|[1-9][0-9]*)(\.[0-9]+)?$');

/// A value as text: decimals without trailing zeros, null as empty.
String displayValue(Object? v) {
  if (v == null) return '';
  if (v is bool) return v ? '✓' : '✗';
  if (v is num || v is String) {
    final d = Decimal.tryParse('$v');
    if (d != null && v is! String) return d.toString();
    return '$v';
  }
  if (v is List) return v.map(displayValue).join(', ');
  return '$v';
}

/// Units per dimension with their factor to the first (canonical) one; mirrors
/// `definition::units` on the server.
const _units = <String, Map<String, String>>{
  'mass': {'kg': '1', 'g': '0.001', 't': '1000'},
  'length': {'m': '1', 'mm': '0.001', 'cm': '0.01', 'km': '1000'},
  'power': {'kW': '1', 'W': '0.001', 'MW': '1000'},
  'voltage': {'V': '1', 'mV': '0.001', 'kV': '1000'},
  'current': {'A': '1', 'mA': '0.001', 'kA': '1000'},
  'speed': {'m/s': '1', 'm/min': '1/60', 'km/h': '1/3.6'},
};

Decimal? _factor(String f) {
  if (f.startsWith('1/')) return null;
  return Decimal.parse(f);
}

/// The factor of [unit] to its dimension's canonical unit, or null if unknown.
String? unitFactor(String? dimension, String unit) => _units[dimension]?[unit];

/// Converts a decimal text from [from] to [to] within [dimension]. Returns the text unchanged when
/// the units are equal or unknown.
String convertUnit(String value, String? dimension, String from, String to) {
  if (from == to) return value;
  final table = _units[dimension];
  final v = Decimal.tryParse(value);
  if (table == null || v == null) return value;
  final f = table[from], t = table[to];
  if (f == null || t == null) return value;
  // value * f / t, with the "1/x" factors as exact divisions.
  var r = v.toRational();
  r = f.startsWith('1/')
      ? r / Decimal.parse(f.substring(2)).toRational()
      : r * _factor(f)!.toRational();
  r = t.startsWith('1/')
      ? r * Decimal.parse(t.substring(2)).toRational()
      : r / _factor(t)!.toRational();
  final d = r.toDecimal(scaleOnInfinitePrecision: 9);
  return _trim(d.toString());
}

String _trim(String s) {
  if (!s.contains('.')) return s;
  s = s.replaceFirst(RegExp(r'0+$'), '');
  return s.endsWith('.') ? s.substring(0, s.length - 1) : s;
}
