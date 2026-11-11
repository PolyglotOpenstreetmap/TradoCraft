import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app/api.dart';
import '../../app/profile.dart';
import '../../app/session.dart';
import '../field/offline/kv_store.dart';
import '../../l10n/l10n.dart';

/// Seeded demo users (SEED_DEV_DATA=true), one per role, for quick switching in dev mode.
const _demoUsers = [
  'admin',
  'sales',
  'manager',
  'support',
  'production',
  'viewer',
];

class LoginPage extends ConsumerStatefulWidget {
  const LoginPage({super.key});

  @override
  ConsumerState<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends ConsumerState<LoginPage> {
  late final _server = TextEditingController(
    text: ref.read(sessionProvider).serverUrl,
  );
  final _user = TextEditingController(text: 'sales@example.com');
  final _token = TextEditingController();
  bool _useToken = false;
  bool _busy = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    // A named profile remembers who signed in last, so a window per role opens ready to go.
    if (ref.read(profileProvider) != null) {
      ref.read(kvStoreProvider).get(SessionStore.lastDevUserKey).then((u) {
        if (u != null && u.isNotEmpty && mounted && !_userEdited) {
          _user.text = u;
        }
      }, onError: (Object _) {});
    }
  }

  /// Set by typing in the field (not by assigning [_user]'s text), so a remembered user never
  /// overwrites what the person typed, even when they typed the default back.
  bool _userEdited = false;

  Future<void> _signIn() async {
    final session = Session(
      serverUrl: _server.text.trim(),
      devUser: _useToken ? null : _user.text.trim(),
      accessToken: _useToken ? _token.text.trim() : null,
    );
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      // Verify the credentials before storing them.
      final probe = ProviderContainer(
        overrides: [initialSessionProvider.overrideWithValue(session)],
      );
      try {
        await probe.read(apiProvider).core.me();
      } finally {
        probe.dispose();
      }
      await ref.read(sessionProvider.notifier).signIn(session);
    } catch (e) {
      setState(() => _error = errorMessage(e));
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        // Scrolls in short windows, so the sign-in button stays reachable.
        child: SingleChildScrollView(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 420),
            child: Card(
              child: Padding(
                padding: const EdgeInsets.all(24),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text(
                      'tradocraft',
                      style: Theme.of(context).textTheme.headlineMedium,
                    ),
                    const SizedBox(height: 24),
                    TextField(
                      controller: _server,
                      decoration: InputDecoration(
                        labelText: context.l10n.server,
                      ),
                    ),
                    SizedBox(height: 12),
                    SegmentedButton<bool>(
                      segments: [
                        ButtonSegment(
                          value: false,
                          label: Text(context.l10n.developmentUser),
                        ),
                        ButtonSegment(
                          value: true,
                          label: Text(context.l10n.accessToken),
                        ),
                      ],
                      selected: {_useToken},
                      onSelectionChanged: (s) =>
                          setState(() => _useToken = s.first),
                    ),
                    const SizedBox(height: 12),
                    if (_useToken)
                      TextField(
                        controller: _token,
                        obscureText: true,
                        decoration: InputDecoration(
                          labelText: context.l10n.oidcAccessToken,
                        ),
                      )
                    else
                      TextField(
                        controller: _user,
                        onChanged: (_) => _userEdited = true,
                        decoration: InputDecoration(
                          labelText: context.l10n.email,
                          helperText: context.l10n.devModeHelp,
                        ),
                        onSubmitted: (_) => _signIn(),
                      ),
                    if (!_useToken) ...[
                      const SizedBox(height: 12),
                      Wrap(
                        spacing: 8,
                        runSpacing: 4,
                        children: [
                          for (final u in _demoUsers)
                            ActionChip(
                              label: Text(u),
                              onPressed: _busy
                                  ? null
                                  : () {
                                      // Keeps a remembered user from replacing this choice.
                                      _userEdited = true;
                                      _user.text = '$u@example.com';
                                      _signIn();
                                    },
                            ),
                        ],
                      ),
                    ],
                    if (_error != null) ...[
                      const SizedBox(height: 12),
                      Text(
                        _error!,
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.error,
                        ),
                      ),
                    ],
                    const SizedBox(height: 20),
                    FilledButton(
                      onPressed: _busy ? null : _signIn,
                      child: Text(context.l10n.signIn),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
