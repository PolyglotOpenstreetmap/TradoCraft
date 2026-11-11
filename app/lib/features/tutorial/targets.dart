import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Knows the widgets a tutorial step can point at, by id.
class TutorialTargetRegistry {
  final _keys = <String, GlobalKey>{};

  /// Makes [key] the widget for [id] (the latest registration wins).
  void register(String id, GlobalKey key) => _keys[id] = key;

  /// Forgets [id] when it still belongs to [key].
  void unregister(String id, GlobalKey key) {
    if (_keys[id] == key) _keys.remove(id);
  }

  /// The key registered for [id].
  GlobalKey? operator [](String id) => _keys[id];
}

/// The registry of the running app.
final tutorialTargetsProvider = Provider<TutorialTargetRegistry>(
  (ref) => TutorialTargetRegistry(),
);

/// Marks [child] as something a tutorial step can spotlight. It changes nothing about layout or
/// behaviour: it only keeps a [GlobalKey] in the registry under [id] while it is on screen.
///
/// Do not wrap an `Expanded`/`Positioned` directly; wrap what is inside it.
class TutorialTarget extends ConsumerStatefulWidget {
  /// Creates a target.
  const TutorialTarget({super.key, required this.id, required this.child});

  /// The id steps refer to (`MissionStep.targets`).
  final String id;

  /// The widget to point at.
  final Widget child;

  @override
  ConsumerState<TutorialTarget> createState() => _TutorialTargetState();
}

/// State of [TutorialTarget]: owns the key and registers it.
class _TutorialTargetState extends ConsumerState<TutorialTarget> {
  final _key = GlobalKey();
  late TutorialTargetRegistry _registry;
  late String _id;

  @override
  void initState() {
    super.initState();
    _registry = ref.read(tutorialTargetsProvider);
    _id = widget.id;
    _registry.register(_id, _key);
  }

  @override
  void didUpdateWidget(TutorialTarget old) {
    super.didUpdateWidget(old);
    if (old.id != widget.id) {
      _registry.unregister(_id, _key);
      _id = widget.id;
      _registry.register(_id, _key);
    }
  }

  @override
  void dispose() {
    _registry.unregister(_id, _key);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) =>
      KeyedSubtree(key: _key, child: widget.child);
}
