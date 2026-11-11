import '../../l10n/l10n.dart';

/// The text of a mission key (see `missionKey` and `stepKey` in `model.dart`) in the language of
/// [l], or the key itself when it is unknown (so a half-written chapter shows its key instead of
/// crashing; the completeness test fails for it).
String tutorialText(AppLocalizations l, String key) =>
    tutorialTextOrNull(l, key) ?? key;

/// Like [tutorialText], but null for an unknown key. ARB keys are compile-time getters, so each
/// mission key is listed in this table.
String? tutorialTextOrNull(AppLocalizations l, String key) => switch (key) {
  'tutWelcomeTitle' => l.tutWelcomeTitle,
  'tutWelcomeSummary' => l.tutWelcomeSummary,
  'tutProductTitle' => l.tutProductTitle,
  'tutProductSummary' => l.tutProductSummary,
  'tutWelcomeBannerTitle' => l.tutWelcomeBannerTitle,
  'tutWelcomeBannerBody' => l.tutWelcomeBannerBody,
  'tutWelcomeTabsTitle' => l.tutWelcomeTabsTitle,
  'tutWelcomeTabsBody' => l.tutWelcomeTabsBody,
  'tutWelcomeControlsTitle' => l.tutWelcomeControlsTitle,
  'tutWelcomeControlsBody' => l.tutWelcomeControlsBody,
  'tutWelcomeMenuTitle' => l.tutWelcomeMenuTitle,
  'tutWelcomeMenuBody' => l.tutWelcomeMenuBody,
  'tutWelcomeProductsTitle' => l.tutWelcomeProductsTitle,
  'tutWelcomeProductsBody' => l.tutWelcomeProductsBody,
  'tutProductOpenTitle' => l.tutProductOpenTitle,
  'tutProductOpenBody' => l.tutProductOpenBody,
  'tutProductMotorTitle' => l.tutProductMotorTitle,
  'tutProductMotorBody' => l.tutProductMotorBody,
  'tutProductMotorHint' => l.tutProductMotorHint,
  'tutProductFrameTitle' => l.tutProductFrameTitle,
  'tutProductFrameBody' => l.tutProductFrameBody,
  'tutProductControllerTitle' => l.tutProductControllerTitle,
  'tutProductControllerBody' => l.tutProductControllerBody,
  'tutProductCartTitle' => l.tutProductCartTitle,
  'tutProductCartBody' => l.tutProductCartBody,
  'tutProductOpenCartTitle' => l.tutProductOpenCartTitle,
  'tutProductOpenCartBody' => l.tutProductOpenCartBody,
  'tutProductBomTabTitle' => l.tutProductBomTabTitle,
  'tutProductBomTabBody' => l.tutProductBomTabBody,
  'tutProductRevisionTitle' => l.tutProductRevisionTitle,
  'tutProductRevisionBody' => l.tutProductRevisionBody,
  'tutProductLinesTitle' => l.tutProductLinesTitle,
  'tutProductLinesBody' => l.tutProductLinesBody,
  'tutProductLinesHint' => l.tutProductLinesHint,
  'tutProductActivateTitle' => l.tutProductActivateTitle,
  'tutProductActivateBody' => l.tutProductActivateBody,
  'tutProductDoneTitle' => l.tutProductDoneTitle,
  'tutProductDoneBody' => l.tutProductDoneBody,
  _ => null,
};
