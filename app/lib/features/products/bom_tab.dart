import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../app/api.dart';
import '../../shared/actions.dart';
import '../../shared/format.dart';
import '../../shared/search_picker.dart';
import '../../l10n/l10n.dart';
import '../../shared/app_icons.dart';

/// An editable line of a draft BOM.
class _EditLine {
  _EditLine({
    required this.productId,
    required this.code,
    required this.name,
    required this.unit,
    String quantity = '1',
    String scrap = '0',
  }) : quantity = TextEditingController(text: quantity),
       scrap = TextEditingController(text: scrap);

  final String productId;
  final String code;
  final String name;
  final String unit;
  final TextEditingController quantity;
  final TextEditingController scrap;
}

/// Bill of materials of a product: revisions, editing the draft, activation and explosion.
class BomTab extends ConsumerStatefulWidget {
  const BomTab({super.key, required this.product});
  final Product product;

  @override
  ConsumerState<BomTab> createState() => _BomTabState();
}

class _BomTabState extends ConsumerState<BomTab> {
  List<BomSummary> _revisions = [];
  Bom? _bom;
  final List<_EditLine> _lines = [];
  final _notes = TextEditingController();
  final _explodeQty = TextEditingController(text: '1');
  Explosion? _explosion;
  bool _loading = true;
  String? _error;

  TradocraftApi get _api => ref.read(apiProvider);
  bool get _isDraft => _bom?.status == 'draft';

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load({String? select}) async {
    setState(() => _loading = true);
    try {
      final revisions = await _api.engineering.listBoms(id: widget.product.id);
      final active = revisions.where((b) => b.status == 'active').firstOrNull;
      final id = select ?? active?.id ?? revisions.firstOrNull?.id;
      final bom = id == null ? null : await _api.engineering.getBom(id: id);
      setState(() {
        _revisions = revisions;
        _setBom(bom);
        _error = null;
      });
      if (active != null && _explosion == null) await _explode();
    } catch (e) {
      setState(() => _error = errorMessage(e));
    } finally {
      if (mounted) setState(() => _loading = false);
    }
  }

  void _setBom(Bom? bom) {
    _bom = bom;
    _notes.text = bom?.notes ?? '';
    _lines
      ..clear()
      ..addAll(
        (bom?.lines ?? const []).map(
          (l) => _EditLine(
            productId: l.componentProductId,
            code: l.componentCode,
            name: l.componentName,
            unit: l.componentUnit,
            quantity: number(l.quantity),
            scrap: number(l.scrapPercent),
          ),
        ),
      );
  }

  Future<void> _select(String id) async {
    final bom = await _api.engineering.getBom(id: id);
    setState(() => _setBom(bom));
  }

  Future<void> _newRevision() async {
    Bom? created;
    final ok = await runAction(context, () async {
      created = await _api.engineering.createBom(
        id: widget.product.id,
        body: const NewBomInput(),
      );
    });
    if (ok) await _load(select: created!.id);
  }

  Future<bool> _saveDraft() async {
    final lines = <BomLineInput>[];
    for (final (i, l) in _lines.indexed) {
      final q = parseDecimalInput(l.quantity.text);
      final s = parseDecimalInput(l.scrap.text.isEmpty ? '0' : l.scrap.text);
      if (q == null || s == null) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(context.l10n.enterValidNumbersLine(i + 1))),
        );
        return false;
      }
      lines.add(
        BomLineInput(
          componentProductId: l.productId,
          quantity: q,
          scrapPercent: s,
        ),
      );
    }
    Bom? saved;
    final ok = await runAction(
      context,
      () async => saved = await _api.engineering.updateBom(
        id: _bom!.id,
        body: BomInput(notes: _notes.text, lines: lines),
      ),
      success: context.l10n.draftSaved,
    );
    if (ok) setState(() => _setBom(saved));
    return ok;
  }

  Future<void> _activate() async {
    if (!await _saveDraft()) return;
    if (!mounted) return;
    final ok = await runAction(
      context,
      () => _api.engineering.activateBom(id: _bom!.id),
      success: context.l10n.revisionActivated,
    );
    if (ok) {
      _explosion = null;
      await _load(select: _bom!.id);
    }
  }

  Future<void> _explode() async {
    final q = parseDecimalInput(_explodeQty.text);
    if (q == null) return;
    try {
      final e = await _api.engineering.explodeBom(
        id: widget.product.id,
        quantity: q,
      );
      setState(() => _explosion = e);
    } catch (e) {
      setState(() => _explosion = null);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_loading && _bom == null) {
      return const Center(child: CircularProgressIndicator());
    }
    final canWrite =
        ref.watch(meProvider).value?.permissions.contains('bom.write') ?? false;
    final theme = Theme.of(context);
    final hasDraft = _revisions.any((r) => r.status == 'draft');
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        if (_error != null)
          Text(_error!, style: TextStyle(color: theme.colorScheme.error)),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            if (_revisions.isEmpty) Text(context.l10n.noBom),
            if (_revisions.isNotEmpty)
              DropdownButton<String>(
                value: _bom?.id,
                items: [
                  for (final r in _revisions)
                    DropdownMenuItem(
                      value: r.id,
                      child: Text(
                        context.l10n.revisionItem(
                          r.revision,
                          context.l10n.bomStatus(r.status),
                          r.lineCount,
                        ),
                      ),
                    ),
                ],
                onChanged: (id) => id == null ? null : _select(id),
              ),
            if (canWrite && !hasDraft)
              OutlinedButton.icon(
                onPressed: _newRevision,
                icon: const Icon(AppIcons.add),
                label: Text(context.l10n.newRevision),
              ),
            if (canWrite && _isDraft) ...[
              OutlinedButton.icon(
                onPressed: _saveDraft,
                icon: const Icon(AppIcons.save),
                label: Text(context.l10n.saveDraft),
              ),
              FilledButton.icon(
                onPressed: _activate,
                icon: const Icon(AppIcons.check),
                label: Text(context.l10n.activate),
              ),
            ],
          ],
        ),
        if (_bom != null) ...[
          const SizedBox(height: 8),
          if (!_isDraft)
            Text(
              _bom!.status == 'active'
                  ? 'Active since ${date(_bom!.activatedAt)}. Changes go through a new revision.'
                  : context.l10n.obsoleteRevision,
              style: theme.textTheme.bodySmall,
            ),
          const SizedBox(height: 8),
          _linesTable(theme, editable: canWrite && _isDraft),
          if (canWrite && _isDraft) ...[
            const SizedBox(height: 8),
            Align(
              alignment: Alignment.centerLeft,
              child: SizedBox(
                width: 420,
                child: SearchPicker<Product>(
                  label: context.l10n.addComponent,
                  clearOnSelect: true,
                  display: (p) => '${p.code} · ${p.name}',
                  search: (q) async =>
                      (await _api.products.listProducts(
                            q: q,
                            active: true,
                            limit: 20,
                          )).items
                          .where(
                            (p) =>
                                p.id != widget.product.id &&
                                p.productType != 'service',
                          )
                          .toList(),
                  onSelected: (p) => setState(
                    () => _lines.add(
                      _EditLine(
                        productId: p.id,
                        code: p.code,
                        name: p.name,
                        unit: p.unit,
                      ),
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(height: 8),
            TextField(
              controller: _notes,
              decoration: InputDecoration(
                labelText: context.l10n.revisionNotes,
              ),
            ),
          ],
        ],
        const Divider(height: 40),
        Text(
          context.l10n.materialRequirements,
          style: theme.textTheme.titleMedium,
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            SizedBox(
              width: 140,
              child: TextField(
                controller: _explodeQty,
                decoration: InputDecoration(
                  labelText: context.l10n.quantity,
                  suffixText: widget.product.unit,
                ),
                onSubmitted: (_) => _explode(),
              ),
            ),
            const SizedBox(width: 12),
            OutlinedButton(
              onPressed: _explode,
              child: Text(context.l10n.explode),
            ),
          ],
        ),
        const SizedBox(height: 12),
        if (_explosion == null)
          Text(context.l10n.noActiveBom)
        else
          _explosionView(theme, _explosion!),
      ],
    );
  }

  Widget _linesTable(ThemeData theme, {required bool editable}) {
    final bom = _bom!;
    return Table(
      columnWidths: const {
        0: FixedColumnWidth(40),
        1: FixedColumnWidth(130),
        2: FlexColumnWidth(),
        3: FixedColumnWidth(130),
        4: FixedColumnWidth(110),
        5: FixedColumnWidth(48),
      },
      defaultVerticalAlignment: TableCellVerticalAlignment.middle,
      children: [
        TableRow(
          children: [
            for (final h in [
              '#',
              context.l10n.component,
              context.l10n.description,
              context.l10n.quantity,
              context.l10n.scrapPercent,
              '',
            ])
              Padding(
                padding: const EdgeInsets.all(6),
                child: Text(h, style: theme.textTheme.labelLarge),
              ),
          ],
        ),
        for (final (i, l) in _lines.indexed)
          TableRow(
            children: [
              Padding(
                padding: const EdgeInsets.all(6),
                child: Text('${i + 1}'),
              ),
              Padding(padding: const EdgeInsets.all(6), child: Text(l.code)),
              Padding(
                padding: const EdgeInsets.all(6),
                child: Row(
                  children: [
                    Flexible(child: Text(l.name)),
                    if (!editable &&
                        bom.lines.elementAtOrNull(i)?.componentHasBom == true)
                      Padding(
                        padding: EdgeInsets.only(left: 6),
                        child: Chip(label: Text(context.l10n.subassembly)),
                      ),
                    if (!editable &&
                        bom.lines.elementAtOrNull(i)?.componentTracking ==
                            'serial')
                      Padding(
                        padding: EdgeInsets.only(left: 6),
                        child: Chip(label: Text(context.l10n.serial)),
                      ),
                  ],
                ),
              ),
              Padding(
                padding: const EdgeInsets.all(4),
                child: editable
                    ? TextField(
                        controller: l.quantity,
                        textAlign: TextAlign.right,
                        decoration: InputDecoration(suffixText: l.unit),
                      )
                    : Text(
                        '${l.quantity.text} ${l.unit}',
                        textAlign: TextAlign.right,
                      ),
              ),
              Padding(
                padding: const EdgeInsets.all(4),
                child: editable
                    ? TextField(controller: l.scrap, textAlign: TextAlign.right)
                    : Text(
                        l.scrap.text == '0' ? '' : '${l.scrap.text} %',
                        textAlign: TextAlign.right,
                      ),
              ),
              editable
                  ? IconButton(
                      tooltip: context.l10n.removeLine,
                      onPressed: () => setState(() => _lines.removeAt(i)),
                      icon: const Icon(AppIcons.close),
                    )
                  : const SizedBox.shrink(),
            ],
          ),
      ],
    );
  }

  Widget _explosionView(ThemeData theme, Explosion e) => Wrap(
    spacing: 32,
    runSpacing: 16,
    children: [
      ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 560),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Structure (revision ${e.bomRevision})',
              style: theme.textTheme.labelLarge,
            ),
            for (final n in e.nodes)
              Padding(
                padding: EdgeInsets.only(left: (n.level - 1) * 24.0, top: 4),
                child: Row(
                  children: [
                    Icon(
                      n.isLeaf ? AppIcons.circleOutlined : AppIcons.accountTree,
                      size: 16,
                    ),
                    const SizedBox(width: 6),
                    Expanded(child: Text('${n.code}  ${n.name}')),
                    Text('${number(n.requiredQuantity)} ${n.unit}'),
                  ],
                ),
              ),
          ],
        ),
      ),
      ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 420),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              context.l10n.totalsToBuyOrMake,
              style: theme.textTheme.labelLarge,
            ),
            for (final r in e.requirements)
              Padding(
                padding: const EdgeInsets.only(top: 4),
                child: Row(
                  children: [
                    Expanded(child: Text('${r.code}  ${r.name}')),
                    Text('${number(r.quantity)} ${r.unit}'),
                  ],
                ),
              ),
          ],
        ),
      ),
    ],
  );
}
