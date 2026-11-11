/// The product's display name: navigation label, login heading and window title.
const brandName = 'TradoCraft';

/// The title of the app window or task: the brand, plus the profile when one is active (several
/// windows, one per role, can then be told apart).
String appWindowTitle(String? profile) =>
    profile == null ? brandName : '$brandName - $profile';
