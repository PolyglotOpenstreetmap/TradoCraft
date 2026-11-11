import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../app/api.dart';
import '../../shared/app_icons.dart';

import '../../l10n/l10n.dart';
import 'modules_page.dart';
import 'price_zones_page.dart';
import 'users_page.dart';

/// User and module administration: one destination with a switch between the two tables.
class AdminPage extends ConsumerStatefulWidget {
  const AdminPage({super.key});

  @override
  ConsumerState<AdminPage> createState() => _AdminPageState();
}

class _AdminPageState extends ConsumerState<AdminPage> {
  String _tab = 'users';

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
                SegmentedButton<String>(
                  showSelectedIcon: false,
                  segments: [
                    ButtonSegment(value: 'users', label: Text(l.adminUsers)),
                    ButtonSegment(
                      value: 'modules',
                      label: Text(l.adminModules),
                    ),
                    ButtonSegment(
                      value: 'zones',
                      label: Text(l.adminPriceZones),
                    ),
                  ],
                  selected: {_tab},
                  onSelectionChanged: (s) => setState(() => _tab = s.first),
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
        Expanded(
          child: switch (_tab) {
            'modules' => const ModulesPage(),
            'zones' => const PriceZonesPage(),
            _ => const UsersPage(),
          },
        ),
      ],
    );
  }
}
