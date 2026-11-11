import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app/api.dart';
import '../../l10n/l10n.dart';
import '../../shared/actions.dart';
import '../../shared/app_icons.dart';
import 'production_page.dart';

/// The production divisions, kept by the administrator (`production.admin`).
class DivisionsPage extends ConsumerWidget {
  const DivisionsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l = context.l10n;
    final admin =
        ref.watch(meProvider).value?.permissions.contains('production.admin') ??
        false;
    final divisions = ref.watch(divisionsProvider);
    Future<void> edit([Division? d]) async {
      final saved = await showDialog<bool>(
        context: context,
        builder: (_) => _DivisionDialog(existing: d),
      );
      if (saved == true) ref.invalidate(divisionsProvider);
    }

    return Scaffold(
      appBar: AppBar(title: Text(l.productionDivisions)),
      floatingActionButton: admin
          ? FloatingActionButton.extended(
              key: const ValueKey('add-division'),
              onPressed: edit,
              icon: const Icon(AppIcons.add),
              label: Text(l.productionDivisionAdd),
            )
          : null,
      body: !admin
          ? Center(child: Text(l.noPermission))
          : divisions.when(
              loading: () => const Center(child: CircularProgressIndicator()),
              error: (e, _) => Center(child: Text(errorMessage(e))),
              data: (list) => ListView(
                padding: const EdgeInsets.all(16),
                children: [
                  for (final d in list)
                    ListTile(
                      key: ValueKey('division-${d.id}'),
                      title: Text('${d.code} · ${d.name}'),
                      subtitle: Text(
                        d.active
                            ? '${d.openOrders}'
                            : '${l.inactive} · ${d.openOrders}',
                      ),
                      trailing: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          IconButton(
                            tooltip: l.edit,
                            icon: const Icon(AppIcons.edit),
                            onPressed: () => edit(d),
                          ),
                          IconButton(
                            key: ValueKey('delete-${d.id}'),
                            tooltip: l.remove,
                            icon: const Icon(AppIcons.delete),
                            onPressed: () async {
                              await runAction(
                                context,
                                () => ref
                                    .read(apiProvider)
                                    .production
                                    .deleteDivision(id: d.id),
                              );
                              ref.invalidate(divisionsProvider);
                            },
                          ),
                        ],
                      ),
                    ),
                ],
              ),
            ),
    );
  }
}

class _DivisionDialog extends ConsumerStatefulWidget {
  const _DivisionDialog({this.existing});
  final Division? existing;

  @override
  ConsumerState<_DivisionDialog> createState() => _DivisionDialogState();
}

class _DivisionDialogState extends ConsumerState<_DivisionDialog> {
  late final _code = TextEditingController(text: widget.existing?.code);
  late final _name = TextEditingController(text: widget.existing?.name);
  late bool _active = widget.existing?.active ?? true;
  String? _error;

  @override
  void dispose() {
    _code.dispose();
    _name.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    final body = DivisionInput(
      code: _code.text.trim(),
      name: _name.text.trim(),
      sortOrder: widget.existing?.sortOrder ?? 0,
      active: _active,
    );
    final api = ref.read(apiProvider).production;
    try {
      final e = widget.existing;
      if (e == null) {
        await api.createDivision(body: body);
      } else {
        await api.updateDivision(id: e.id, body: body);
      }
      if (mounted) Navigator.pop(context, true);
    } catch (e) {
      setState(() => _error = errorMessage(e));
    }
  }

  @override
  Widget build(BuildContext context) {
    final l = context.l10n;
    return AlertDialog(
      title: Text(
        widget.existing == null
            ? l.productionDivisionAdd
            : l.productionDivisionEdit,
      ),
      content: SizedBox(
        width: 380,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextField(
              key: const ValueKey('division-code'),
              controller: _code,
              decoration: InputDecoration(labelText: l.code),
            ),
            TextField(
              key: const ValueKey('division-name'),
              controller: _name,
              decoration: InputDecoration(labelText: l.name),
            ),
            SwitchListTile(
              contentPadding: EdgeInsets.zero,
              title: Text(l.active),
              value: _active,
              onChanged: (v) => setState(() => _active = v),
            ),
            if (_error != null)
              Text(
                _error!,
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(l.cancel),
        ),
        FilledButton(
          key: const ValueKey('division-save'),
          onPressed: _save,
          child: Text(l.save),
        ),
      ],
    );
  }
}
