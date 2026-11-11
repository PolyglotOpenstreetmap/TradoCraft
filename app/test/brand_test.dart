import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/app/brand.dart';

void main() {
  test('the window title is the brand, plus the profile when there is one', () {
    expect(appWindowTitle(null), 'TradoCraft');
    expect(appWindowTitle('admin'), 'TradoCraft - admin');
  });
}
