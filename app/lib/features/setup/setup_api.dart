import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// What a server says about being set up (`GET /api/v1/setup/status`, no sign-in needed).
class SetupStatusInfo {
  /// Creates the answer.
  const SetupStatusInfo({
    required this.needsSetup,
    required this.tokenRequired,
    required this.authMode,
  });

  /// No company exists yet and the server allows setting one up.
  final bool needsSetup;

  /// The server wants its `BOOTSTRAP_TOKEN` with the request.
  final bool tokenRequired;

  /// `dev` (the admin e-mail becomes the dev user) or `oidc`.
  final String authMode;

  /// True when the administrator can be signed in directly (development authentication).
  bool get isDev => authMode == 'dev';
}

/// The company and administrator to create (`POST /api/v1/setup/bootstrap`).
class SetupRequest {
  /// Creates a request; optional fields are left out of the body when blank.
  const SetupRequest({
    required this.name,
    required this.legalName,
    required this.vatNumber,
    required this.countryCode,
    required this.currency,
    required this.defaultLanguage,
    required this.adminEmail,
    required this.adminDisplayName,
    this.token,
  });

  /// Trading name.
  final String name;

  /// Registered name, blank when the same.
  final String legalName;

  /// VAT number, blank when none.
  final String vatNumber;

  /// ISO country code, two letters.
  final String countryCode;

  /// ISO currency code, three letters.
  final String currency;

  /// One of nl, fr, en, de, es.
  final String defaultLanguage;

  /// E-mail of the first administrator.
  final String adminEmail;

  /// Display name of the first administrator.
  final String adminDisplayName;

  /// The server's `BOOTSTRAP_TOKEN`, when it asks for one.
  final String? token;

  /// The JSON body.
  Map<String, Object?> toJson() => {
    'name': name.trim(),
    if (legalName.trim().isNotEmpty) 'legal_name': legalName.trim(),
    if (vatNumber.trim().isNotEmpty) 'vat_number': vatNumber.trim(),
    'country_code': countryCode.trim().toUpperCase(),
    'currency': currency.trim().toUpperCase(),
    'default_language': defaultLanguage,
    'admin_email': adminEmail.trim(),
    'admin_display_name': adminDisplayName.trim(),
    if (token != null && token!.trim().isNotEmpty) 'token': token!.trim(),
  };
}

/// What the server created.
class SetupResult {
  /// Creates the result.
  const SetupResult({
    required this.companyName,
    required this.adminEmail,
    required this.authMode,
  });

  /// Name of the new company.
  final String companyName;

  /// E-mail of the administrator as stored (lower case).
  final String adminEmail;

  /// `dev` or `oidc`.
  final String authMode;

  /// True when the app can sign the administrator in directly.
  bool get isDev => authMode == 'dev';
}

/// Makes an HTTP client for a server address, without any session headers (the setup endpoints
/// need no sign-in). Overridden in tests with a fake.
final setupDioProvider = Provider<Dio Function(String serverUrl)>(
  (ref) =>
      (url) => Dio(
        BaseOptions(
          baseUrl: url,
          connectTimeout: const Duration(seconds: 8),
          receiveTimeout: const Duration(seconds: 30),
        ),
      ),
);

/// Asks a server whether it needs setting up.
final setupStatusProvider =
    Provider<Future<SetupStatusInfo> Function(String serverUrl)>((ref) {
      final make = ref.watch(setupDioProvider);
      return (url) async {
        final dio = make(url);
        try {
          final res = await dio.get<Object?>('/api/v1/setup/status');
          final d = res.data;
          if (d is! Map || d['needs_setup'] is! bool) {
            throw const FormatException('Not a TradoCraft server');
          }
          return SetupStatusInfo(
            needsSetup: d['needs_setup'] as bool,
            tokenRequired: d['token_required'] == true,
            authMode: '${d['auth_mode'] ?? 'oidc'}',
          );
        } finally {
          dio.close();
        }
      };
    });

/// Creates the company on a server. Throws a [DioException] for 403 (token), 404 (disabled),
/// 409 (already set up) and 422 (invalid input).
final setupBootstrapProvider =
    Provider<Future<SetupResult> Function(String serverUrl, SetupRequest req)>((
      ref,
    ) {
      final make = ref.watch(setupDioProvider);
      return (url, req) async {
        final dio = make(url);
        try {
          final res = await dio.post<Object?>(
            '/api/v1/setup/bootstrap',
            data: req.toJson(),
          );
          final d = res.data as Map;
          return SetupResult(
            companyName: '${(d['company'] as Map)['name']}',
            adminEmail: '${(d['admin'] as Map)['email']}',
            authMode: '${d['auth_mode'] ?? 'oidc'}',
          );
        } finally {
          dio.close();
        }
      };
    });

/// Validation shared by the wizard's steps; mirrors the backend's rules.
class SetupValidation {
  const SetupValidation._();

  /// A company name: not blank, at most 200 characters.
  static bool name(String v) => v.trim().isNotEmpty && v.trim().length <= 200;

  /// A VAT number is optional; when given it has 4 to 20 letters and digits (spaces, dots and
  /// dashes are ignored).
  static bool vat(String v) {
    final t = v.replaceAll(RegExp(r'[ .\-]'), '');
    if (v.trim().isEmpty) return true;
    return RegExp(r'^[A-Za-z0-9]{4,20}$').hasMatch(t);
  }

  /// A two-letter country code.
  static bool country(String v) => RegExp(r'^[A-Za-z]{2}$').hasMatch(v.trim());

  /// A three-letter currency code.
  static bool currency(String v) => RegExp(r'^[A-Za-z]{3}$').hasMatch(v.trim());

  /// An e-mail address: one `@`, a dot in the domain, no spaces.
  static bool email(String v) {
    final t = v.trim();
    return t.length <= 254 &&
        RegExp(r'^[^\s@]+@[^\s@.]+(\.[^\s@.]+)+$').hasMatch(t);
  }
}
