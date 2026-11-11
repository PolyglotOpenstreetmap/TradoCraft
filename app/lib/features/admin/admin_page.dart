import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../app/api.dart';
import '../../shared/app_icons.dart';

import '../../l10n/l10n.dart';
import 'modules_page.dart';
import 'users_page.dart';

/// User and module administration: one destination with a switch between the two tables.
class AdminPage extends ConsumerStatefulWidget {
  const AdminPage({super.key});

  @override
  ConsumerState<AdminPage> createState() => _AdminPageState();
}

class _AdminPageState extends ConsumerState<AdminPage> {
  bool _users = true;

  @override
  Widget build(BuildContext context) {
    final l = context.l10n;
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
          child: Align(
            alignment: Alignment.centerLeft,
            child: Wrap(
              spacing: 12,
              runSpacing: 8,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: [
                SegmentedButton<bool>(
                  showSelectedIcon: false,
                  segments: [
                    ButtonSegment(value: true, label: Text(l.adminUsers)),
                    ButtonSegment(value: false, label: Text(l.adminModules)),
                  ],
                  selected: {_users},
                  onSelectionChanged: (s) => setState(() => _users = s.first),
                ),
                // The configurator builder lives with the configurators; designers start here too.
                if (can(ref, 'configurator.design'))
                  OutlinedButton.icon(
                    key: const ValueKey('admin-configurators'),
                    onPressed: () => context.push('/configurator/builder'),
                    icon: const Icon(AppIcons.dashboardCustomize),
                    label: Text(l.cfgbDesignTitle),
                  ),
              ],
            ),
          ),
        ),
        Expanded(child: _users ? const UsersPage() : const ModulesPage()),
      ],
    );
  }
}
