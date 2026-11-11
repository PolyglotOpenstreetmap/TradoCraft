import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app/api.dart';
import '../../l10n/l10n.dart';
import '../../shared/actions.dart';

/// The module catalogue with the company's switches.
final adminModulesProvider = FutureProvider.autoDispose<List<ModuleInfo>>(
  (ref) => ref.watch(apiProvider).core.getModules(),
);

/// Company-wide module switches. The server validates dependencies and answers 422 with a message.
class ModulesPage extends ConsumerStatefulWidget {
  const ModulesPage({super.key});

  @override
  ConsumerState<ModulesPage> createState() => _ModulesPageState();
}

class _ModulesPageState extends ConsumerState<ModulesPage> {
  /// The switches being edited; null until the catalogue has loaded.
  Set<String>? _disabled;
  Set<String> _saved = {};
  bool _saving = false;

  Set<String> _offIn(List<ModuleInfo> list) => {
    for (final m in list)
      if (!m.enabled) m.code,
  };

  Future<void> _save() async {
    setState(() => _saving = true);
    ModulesInput body = ModulesInput(
      disabledModules: _disabled!.toList()..sort(),
    );
    List<ModuleInfo>? result;
    final ok = await runAction(context, () async {
      result = await ref.read(apiProvider).core.setModules(body: body);
    }, success: context.l10n.modulesSaved);
    if (!mounted) return;
    setState(() => _saving = false);
    if (ok && result != null) {
      setState(() {
        _saved = _offIn(result!);
        _disabled = {..._saved};
      });
      ref.invalidate(adminModulesProvider);
      // The server filters permissions by module, so the navigation changes with this.
      ref.invalidate(meProvider);
    }
  }

  @override
  Widget build(BuildContext context) {
    final l = context.l10n;
    final modules = ref.watch(adminModulesProvider);
    return modules.when(
      loading: () => const LinearProgressIndicator(),
      error: (e, _) => Padding(
        padding: const EdgeInsets.all(16),
        child: Text(errorMessage(e)),
      ),
      data: (list) {
        if (_disabled == null) {
          _saved = _offIn(list);
          _disabled = {..._saved};
        }
        final off = _disabled!;
        final changed = off.length != _saved.length || !off.containsAll(_saved);
        return ListView(
          padding: const EdgeInsets.all(16),
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    l.adminModules,
                    style: Theme.of(context).textTheme.headlineSmall,
                  ),
                ),
                FilledButton(
                  onPressed: changed && !_saving ? _save : null,
                  child: Text(l.save),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(l.modulesIntro),
            const SizedBox(height: 8),
            for (final m in list)
              SwitchListTile(
                key: ValueKey('module-${m.code}'),
                title: Text(l.moduleName(m.code)),
                subtitle: m.requires.isEmpty
                    ? null
                    : Text(
                        l.moduleRequires(
                          m.requires.map(l.moduleName).join(', '),
                        ),
                      ),
                value: !off.contains(m.code),
                onChanged: _saving
                    ? null
                    : (on) => setState(() {
                        if (on) {
                          off.remove(m.code);
                        } else {
                          off.add(m.code);
                        }
                      }),
              ),
          ],
        );
      },
    );
  }
}
