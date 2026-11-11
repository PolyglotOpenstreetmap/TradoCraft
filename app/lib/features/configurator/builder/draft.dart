/// The builder's editable form of a configurator definition (ADR 0034).
///
/// The definition is kept as the JSON document itself, so what the server stores, exports and
/// imports comes back exactly as it was loaded: editors change single properties in place and
/// remove properties that are cleared (the server's defaults then apply). Helpers here are pure
/// Dart so they can be tested without widgets: JSON paths to editor locations, the names a formula
/// can use, and CSV for lookup tables.
library;

typedef Json = Map<String, Object?>;

/// A deep copy of a JSON value with modifiable maps and lists.
Object? deepCopy(Object? v) => switch (v) {
  Map m => <String, Object?>{
    for (final e in m.entries) '${e.key}': deepCopy(e.value),
  },
  List l => [for (final e in l) deepCopy(e)],
  _ => v,
};

/// Structural equality of JSON values (maps regardless of key order).
bool jsonEquals(Object? a, Object? b) {
  if (a is Map && b is Map) {
    if (a.length != b.length) return false;
    for (final k in a.keys) {
      if (!b.containsKey(k) || !jsonEquals(a[k], b[k])) return false;
    }
    return true;
  }
  if (a is List && b is List) {
    if (a.length != b.length) return false;
    for (var i = 0; i < a.length; i++) {
      if (!jsonEquals(a[i], b[i])) return false;
    }
    return true;
  }
  return a == b;
}

Json asJson(Object? v) => v is Map<String, Object?>
    ? v
    : v is Map
    ? v.cast<String, Object?>()
    : <String, Object?>{};

/// The maps of a list property (read only; an absent list is empty).
List<Json> jsonList(Json parent, String key) {
  final v = parent[key];
  return v is List ? [for (final e in v) asJson(e)] : const [];
}

/// The list property itself, created when absent (for adding).
List<Object?> mutableList(Json parent, String key) {
  final v = parent[key];
  if (v is List<Object?>) return v;
  final l = <Object?>[...?(v as List?)];
  parent[key] = l;
  return l;
}

/// The map property itself, created when absent.
Json mutableMap(Json parent, String key) {
  final v = parent[key];
  if (v is Map<String, Object?>) return v;
  final m = <String, Object?>{...asJson(v)};
  parent[key] = m;
  return m;
}

/// Sets [key] or removes it when [value] is null, an empty string, list or map (the server's
/// default then applies, and an untouched document serialises unchanged).
void setProp(Json obj, String key, Object? value) {
  final empty =
      value == null ||
      (value is String && value.isEmpty) ||
      (value is List && value.isEmpty) ||
      (value is Map && value.isEmpty);
  if (empty) {
    obj.remove(key);
  } else {
    obj[key] = value;
  }
}

/// Removes a list property when it became empty.
void dropEmpty(Json obj, String key) {
  final v = obj[key];
  if (v is List && v.isEmpty) obj.remove(key);
}

String? str(Object? v) => v is String ? v : null;

/// A number as it is typed: an int when whole and written without a point, else a double; null
/// when empty; the text itself when it is not a number.
Object? parseNumberText(String text) {
  final t = text.trim();
  if (t.isEmpty) return null;
  final i = int.tryParse(t);
  if (i != null) return i;
  final d = double.tryParse(t.replaceAll(',', '.'));
  return d ?? t;
}

String numberText(Object? v) => v == null ? '' : '$v';

// ---------------------------------------------------------------------------------------------
// Scopes
// ---------------------------------------------------------------------------------------------

/// A scope of the definition: the project, the group, or an item table (by index).
class ScopeSel {
  const ScopeSel.project() : kind = 'project', table = null;
  const ScopeSel.group() : kind = 'group', table = null;
  const ScopeSel.item(int this.table) : kind = 'item';

  final String kind;
  final int? table;

  bool get isItem => kind == 'item';

  /// The `scope` value of the API (`project`, `group`, `item`).
  String get apiScope => kind;

  /// The JSON path of the scope object.
  String get path => isItem
      ? r'$.item_tables['
            '$table]'
      : '\$.$kind';

  @override
  bool operator ==(Object other) =>
      other is ScopeSel && other.kind == kind && other.table == table;

  @override
  int get hashCode => Object.hash(kind, table);

  @override
  String toString() => isItem ? 'item:$table' : kind;
}

// ---------------------------------------------------------------------------------------------
// The draft
// ---------------------------------------------------------------------------------------------

class DefinitionDraft {
  DefinitionDraft(Object? raw) : json = asJson(deepCopy(raw)) {
    json.putIfAbsent('name', () => '');
  }

  final Json json;

  Json toJson() => asJson(deepCopy(json));

  String get name => str(json['name']) ?? '';

  /// The object of a scope (project and group are created when absent; an unknown item table
  /// gives an empty, detached map).
  Json scope(ScopeSel s) {
    if (!s.isItem) return mutableMap(json, s.kind);
    final tables = jsonList(json, 'item_tables');
    return s.table! < tables.length ? tables[s.table!] : <String, Object?>{};
  }

  /// Reads a scope without creating it.
  Json readScope(ScopeSel s) => s.isItem ? scope(s) : asJson(json[s.kind]);

  List<Json> itemTables() => jsonList(json, 'item_tables');
  List<Json> tables() => jsonList(json, 'tables');
  List<Json> outputs() => jsonList(json, 'outputs');
  List<Json> validations() => jsonList(json, 'validations');
  List<Json> sections() => jsonList(json, 'sections');

  /// Scopes in display order: project, group, then the item tables.
  List<ScopeSel> scopes() => [
    const ScopeSel.project(),
    const ScopeSel.group(),
    for (var i = 0; i < itemTables().length; i++) ScopeSel.item(i),
  ];

  /// Item table names.
  List<String> itemTableNames() => [
    for (final t in itemTables()) str(t['name']) ?? '',
  ];

  List<String> tableNames() => [for (final t in tables()) str(t['name']) ?? ''];

  /// Columns of a lookup table by name.
  List<String> tableColumns(String table) {
    for (final t in tables()) {
      if (t['name'] == table) {
        return [for (final c in jsonList(t, 'columns')) str(c['name']) ?? ''];
      }
    }
    return const [];
  }

  /// The output rule at [path] (indexes from the top-level list through `children`).
  Json? rule(List<int> path) {
    var list = outputs();
    Json? found;
    for (final i in path) {
      if (i < 0 || i >= list.length) return null;
      found = list[i];
      list = jsonList(found, 'children');
    }
    return found;
  }

  /// The list holding the rule at [path] (created when absent).
  List<Object?> ruleSiblings(List<int> path) {
    if (path.length <= 1) return mutableList(json, 'outputs');
    return mutableList(rule(path.sublist(0, path.length - 1))!, 'children');
  }

  /// The scope a rule's formulas are checked in: its top-level ancestor's scope and table.
  ScopeSel ruleScope(List<int> path) {
    final top = rule(path.take(1).toList());
    return scopeOf(top?['scope'], top?['table']);
  }

  /// A `scope`/`table` pair of a rule or validation as a [ScopeSel] (item tables by name; an item
  /// rule without a table means the only table).
  ScopeSel scopeOf(Object? scope, Object? table) {
    switch (scope) {
      case 'group':
        return const ScopeSel.group();
      case 'item':
        final names = itemTableNames();
        final i = table is String
            ? names.indexOf(table)
            : (names.length == 1 ? 0 : -1);
        return ScopeSel.item(i < 0 ? 0 : i);
      default:
        return const ScopeSel.project();
    }
  }

  /// A name that is not used yet in [existing]: `base`, `base_2`, ...
  static String freeName(String base, Iterable<String> existing) {
    final used = existing.toSet();
    if (!used.contains(base)) return base;
    for (var i = 2; ; i++) {
      if (!used.contains('${base}_$i')) return '${base}_$i';
    }
  }

  /// Every field and computed name of the definition (for unique new names).
  Iterable<String> allValueNames() sync* {
    for (final s in scopes()) {
      final o = readScope(s);
      for (final f in jsonList(o, 'fields')) {
        yield str(f['name']) ?? '';
      }
      for (final c in jsonList(o, 'computed')) {
        yield str(c['name']) ?? '';
      }
    }
  }

  /// Every rule id (children included).
  Iterable<String> allRuleIds() sync* {
    Iterable<String> walk(List<Json> rules) sync* {
      for (final r in rules) {
        yield str(r['id']) ?? '';
        yield* walk(jsonList(r, 'children'));
      }
    }

    yield* walk(outputs());
  }

  // ---------------------------------------------------------------------------------------
  // Names for the formula editor
  // ---------------------------------------------------------------------------------------

  /// The names a formula in [scope] can use, with what they are.
  List<NameHint> namesFor(ScopeSel scope, {bool inRule = false}) {
    final out = <NameHint>[];
    final seen = <String>{};
    void add(String name, String kind, [String detail = '']) {
      if (name.isEmpty || !seen.add(name)) return;
      out.add(NameHint(name, kind, detail));
    }

    void fromScope(ScopeSel s) {
      final o = readScope(s);
      for (final f in jsonList(o, 'fields')) {
        add(str(f['name']) ?? '', 'field', str(f['type']) ?? '');
      }
      for (final c in jsonList(o, 'computed')) {
        add(str(c['name']) ?? '', 'computed', str(c['label']) ?? '');
      }
    }

    // Inner scopes first: a group field overrides the project field of the same name.
    if (scope.isItem) fromScope(scope);
    if (scope.isItem || scope.kind == 'group') {
      fromScope(const ScopeSel.group());
    }
    fromScope(const ScopeSel.project());
    if (scope.isItem) {
      add('index', 'local');
      add('group_name', 'local');
      add('label', 'local');
    } else {
      for (final t in itemTableNames()) {
        add(t, 'items');
      }
      if (itemTables().length == 1) add('items', 'items');
      add('groups', 'local');
      add('group_name', 'local');
    }
    if (inRule) {
      add('each', 'local');
      add('each_index', 'local');
      add('parent', 'local');
    }
    for (final t in tableNames()) {
      add(t, 'table');
    }
    return out;
  }
}

/// A name a formula can use: a field, computed value, item list, local or lookup table.
class NameHint {
  const NameHint(this.name, this.kind, [this.detail = '']);
  final String name;

  /// field | computed | items | local | table | function
  final String kind;
  final String detail;
}

// ---------------------------------------------------------------------------------------------
// Editor locations (JSON paths of server errors)
// ---------------------------------------------------------------------------------------------

/// The builder's tabs, in order.
enum EditorTab {
  general,
  fields,
  tables,
  computed,
  outputs,
  validations,
  test,
  transfer,
}

/// Where in the editor a JSON path points: the tab and the selected element there.
class EditorLocation {
  const EditorLocation(
    this.tab, {
    this.scope,
    this.index,
    this.rule = const [],
    this.row,
    this.property,
  });

  final EditorTab tab;

  /// Fields and computed values: their scope ...
  final ScopeSel? scope;

  /// ... and index; tables and validations: their index.
  final int? index;

  /// Output rules: the path through `children`.
  final List<int> rule;

  /// Table rows.
  final int? row;

  /// The property, e.g. `expr`, `quantity`, `name`.
  final String? property;

  @override
  String toString() =>
      'EditorLocation($tab, scope: $scope, index: $index, rule: $rule, row: $row, property: $property)';
}

final _segment = RegExp(r'\.([A-Za-z_][A-Za-z0-9_]*)|\[(\d+)\]');

/// Splits `$.outputs[1].children[0].quantity` into `['outputs', 1, 'children', 0, 'quantity']`.
List<Object> pathSegments(String path) => [
  for (final m in _segment.allMatches(path))
    if (m.group(1) != null) m.group(1)! else int.parse(m.group(2)!),
];

/// Maps a JSON path of the definition (as server errors report them) to the editor.
EditorLocation locate(String path) {
  final s = pathSegments(path);
  Object? at(int i) => i < s.length ? s[i] : null;
  String? prop(int i) => at(i) is String ? at(i) as String : null;
  switch (at(0)) {
    case 'project' || 'group':
      final scope = at(0) == 'project'
          ? const ScopeSel.project()
          : const ScopeSel.group();
      if (at(1) == 'computed' && at(2) is int) {
        return EditorLocation(
          EditorTab.computed,
          scope: scope,
          index: at(2) as int,
          property: prop(3),
        );
      }
      return EditorLocation(
        EditorTab.fields,
        scope: scope,
        index: at(1) == 'fields' && at(2) is int ? at(2) as int : null,
        property: at(1) == 'fields' ? prop(3) : prop(1),
      );
    case 'item_tables' when at(1) is int:
      final scope = ScopeSel.item(at(1) as int);
      if (at(2) == 'computed' && at(3) is int) {
        return EditorLocation(
          EditorTab.computed,
          scope: scope,
          index: at(3) as int,
          property: prop(4),
        );
      }
      return EditorLocation(
        EditorTab.fields,
        scope: scope,
        index: at(2) == 'fields' && at(3) is int ? at(3) as int : null,
        property: at(2) == 'fields' ? prop(4) : prop(2),
      );
    case 'tables' when at(1) is int:
      return EditorLocation(
        EditorTab.tables,
        index: at(1) as int,
        row: at(2) == 'rows' && at(3) is int ? at(3) as int : null,
        property: at(2) == 'rows' ? prop(4) : prop(2),
      );
    case 'outputs' when at(1) is int:
      final rule = <int>[at(1) as int];
      var i = 2;
      while (at(i) == 'children' && at(i + 1) is int) {
        rule.add(at(i + 1) as int);
        i += 2;
      }
      return EditorLocation(EditorTab.outputs, rule: rule, property: prop(i));
    case 'validations' when at(1) is int:
      return EditorLocation(
        EditorTab.validations,
        index: at(1) as int,
        property: prop(2),
      );
    default:
      return EditorLocation(EditorTab.general, property: prop(0));
  }
}

// ---------------------------------------------------------------------------------------------
// CSV for lookup tables
// ---------------------------------------------------------------------------------------------

String _csvCell(Object? v) {
  final s = v == null ? '' : '$v';
  if (s.contains(RegExp('[",;\n\r]'))) return '"${s.replaceAll('"', '""')}"';
  return s;
}

/// A lookup table as CSV: a header with the column names, then one line per row.
String tableToCsv(Json table) {
  final cols = [
    for (final c in jsonList(table, 'columns')) str(c['name']) ?? '',
  ];
  final b = StringBuffer()..writeln(cols.map(_csvCell).join(','));
  for (final r in jsonList(table, 'rows')) {
    b.writeln([for (final c in cols) _csvCell(r[c])].join(','));
  }
  return b.toString();
}

/// Splits CSV text into records (quotes, doubled quotes and line breaks in quotes; `,` or `;`
/// as the separator, whichever the header uses). A quoted cell that is never closed throws a
/// [FormatException], so that the rest of the file is not read into that cell.
List<List<String>> parseCsv(String text) {
  var t = text;
  if (t.startsWith('﻿')) t = t.substring(1);
  final firstLine = t.split('\n').first;
  final sep = firstLine.contains(';') && !firstLine.contains(',') ? ';' : ',';
  final records = <List<String>>[];
  var record = <String>[];
  final cell = StringBuffer();
  var quoted = false;
  var line = 1;
  var quoteLine = 0;
  for (var i = 0; i < t.length; i++) {
    final ch = t[i];
    if (ch == '\n') line++;
    if (quoted) {
      if (ch == '"') {
        if (i + 1 < t.length && t[i + 1] == '"') {
          cell.write('"');
          i++;
        } else {
          quoted = false;
        }
      } else {
        cell.write(ch);
      }
    } else if (ch == '"') {
      quoted = true;
      quoteLine = line;
    } else if (ch == sep) {
      record.add(cell.toString());
      cell.clear();
    } else if (ch == '\n' || ch == '\r') {
      if (ch == '\r' && i + 1 < t.length && t[i + 1] == '\n') i++;
      record.add(cell.toString());
      cell.clear();
      if (!(record.length == 1 && record.first.isEmpty)) records.add(record);
      record = <String>[];
    } else {
      cell.write(ch);
    }
  }
  if (quoted) {
    throw FormatException('line $quoteLine: a quoted cell is not closed');
  }
  if (cell.isNotEmpty || record.isNotEmpty) {
    record.add(cell.toString());
    if (!(record.length == 1 && record.first.isEmpty)) records.add(record);
  }
  return records;
}

/// A cell typed by its column: numbers and integers as numbers, booleans as `true`/`false`; empty
/// is null. Text that does not fit the type is returned as an [CsvCellError].
Object? typedCell(String text, String? type) {
  final t = text.trim();
  if (t.isEmpty) return null;
  switch (type) {
    case 'number':
      final v = parseNumberText(t);
      return v is num ? v : CsvCellError(t);
    case 'integer':
      final v = int.tryParse(t);
      return v ?? CsvCellError(t);
    case 'boolean':
      final l = t.toLowerCase();
      if (const ['true', '1', 'yes', 'ja', 'oui', 'si', 'sí'].contains(l)) {
        return true;
      }
      if (const ['false', '0', 'no', 'nee', 'non', 'nein'].contains(l)) {
        return false;
      }
      return CsvCellError(t);
    default:
      return text;
  }
}

class CsvCellError {
  const CsvCellError(this.text);
  final String text;
}

/// The result of reading CSV into a table's rows.
class CsvRows {
  const CsvRows(this.rows, this.errors);
  final List<Json> rows;

  /// Problems as (1-based line, column, text) for the person to fix.
  final List<String> errors;
}

/// Reads CSV with a header row into rows of [table] (by its columns). Unknown header columns
/// and values that do not fit the column type are reported; nothing is changed.
CsvRows csvToRows(String csv, Json table) {
  final types = {
    for (final c in jsonList(table, 'columns'))
      str(c['name']) ?? '': str(c['type']),
  };
  final List<List<String>> records;
  try {
    records = parseCsv(csv);
  } on FormatException catch (e) {
    return CsvRows(const [], [e.message]);
  }
  if (records.isEmpty) return const CsvRows([], ['empty file']);
  final header = [for (final h in records.first) h.trim()];
  final errors = <String>[
    for (final h in header)
      if (!types.containsKey(h)) 'unknown column "$h"',
  ];
  final rows = <Json>[];
  for (var r = 1; r < records.length; r++) {
    final row = <String, Object?>{};
    for (var c = 0; c < header.length && c < records[r].length; c++) {
      final name = header[c];
      if (!types.containsKey(name)) continue;
      final v = typedCell(records[r][c], types[name]);
      if (v is CsvCellError) {
        errors.add('line ${r + 1}, $name: "${v.text}"');
      } else if (v != null) {
        row[name] = v;
      }
    }
    rows.add(row);
  }
  return CsvRows(rows, errors);
}
