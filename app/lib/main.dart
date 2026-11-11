import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'app/router.dart';
import 'l10n/l10n.dart';
import 'app/profile.dart';
import 'app/session.dart';
import 'app/zoom.dart';
import 'features/field/offline/kv_store.dart';

/// The storage of [profile]; when its file cannot be opened the session runs on in memory
/// (like [PrefsKvStore] does) so the app still starts, only without remembering anything.
Future<KvStore> _openStore(String? profile) async {
  if (profile == null) return PrefsKvStore();
  try {
    return await JsonFileKvStore.openProfile(profile);
  } catch (_) {
    return MemoryKvStore();
  }
}

Future<void> main(List<String> args) async {
  WidgetsFlutterBinding.ensureInitialized();
  final profile = parseProfile(args);
  // A profile keeps everything in a file of its own; without one the platform preferences are used.
  final KvStore store = await _openStore(profile);
  final session = await SessionStore.load(store);
  runApp(
    ProviderScope(
      overrides: [
        profileProvider.overrideWithValue(profile),
        kvStoreProvider.overrideWithValue(store),
        initialSessionProvider.overrideWithValue(session),
      ],
      child: const TradocraftApp(),
    ),
  );
}

class TradocraftApp extends ConsumerWidget {
  const TradocraftApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MaterialApp.router(
      title: 'tradocraft',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorSchemeSeed: const Color(0xFF2F5D62),
        visualDensity: VisualDensity.compact,
        inputDecorationTheme: const InputDecorationTheme(
          border: OutlineInputBorder(),
          isDense: true,
        ),
      ),
      builder: (context, child) => ZoomShell(child: child ?? const SizedBox()),
      routerConfig: ref.watch(routerProvider),
      locale: ref.watch(localeProvider),
      supportedLocales: AppLocalizations.supportedLocales,
      localizationsDelegates: AppLocalizations.localizationsDelegates,
      onGenerateTitle: (context) {
        final profile = ref.read(profileProvider);
        return profile == null ? 'tradocraft' : 'tradocraft - $profile';
      },
    );
  }
}
