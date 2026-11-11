import 'package:flutter_test/flutter_test.dart';
import 'package:tradocraft_app/shared/format.dart';

void main() {
  test('money uses Belgian notation', () {
    expect(money('1234.5'), '€ 1.234,50');
    expect(money('0'), '€ 0,00');
    expect(money('1234567.891'), '€ 1.234.567,89');
    expect(money('-12.3', currency: 'USD'), 'USD -12,30');
    expect(money(null), '');
  });

  test('number drops trailing zeros', () {
    expect(number('2.0000'), '2');
    expect(number('12.5000'), '12,5');
    expect(number('1000'), '1.000');
  });

  test('decimal input accepts comma and dot', () {
    expect(parseDecimalInput('2,5'), '2.5');
    expect(parseDecimalInput(' 10 '), '10');
    expect(parseDecimalInput('abc'), isNull);
    expect(parseDecimalInput(''), isNull);
  });
}
