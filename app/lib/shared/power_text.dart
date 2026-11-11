/// Short, language-neutral text for a supply profile or a grid, e.g. `AC 230 V 1ph 50 Hz ±10 %`.
String powerProfileText({
  String? current,
  required Object voltage,
  Object? phases,
  Object? frequency,
  Object? tolerance,
}) {
  final parts = [
    if (current != null) current.toUpperCase(),
    '$voltage V',
    if (phases != null) '${phases}ph',
    if (frequency != null) '$frequency Hz',
    if (tolerance != null) '±$tolerance %',
  ];
  return parts.join(' ');
}

/// Formats a supply or grid as it comes in the `details` of a warning (a decoded JSON map).
String powerProfileFromJson(Object? json) {
  if (json is! Map) return '$json';
  return powerProfileText(
    current: json['current_type'] as String?,
    voltage: json['voltage_v'] ?? '?',
    phases: json['phases'],
    frequency: json['frequency_hz'],
    tolerance: json['tolerance_pct'],
  );
}

/// A list of profiles joined for a sentence; `-` when empty.
String powerProfilesText(Iterable<String> profiles) =>
    profiles.isEmpty ? '-' : profiles.join('; ');
