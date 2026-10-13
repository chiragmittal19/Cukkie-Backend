use crate::domain::{PreferenceGroupConfig, PreferenceOptionItem, PreferencesConfig};
use async_trait::async_trait;
use common::{AppError, Language};
use std::collections::HashMap;

#[async_trait]
pub trait ConfigRepository: Send + Sync {
    async fn get_preferences_config(&self, lang: Language) -> Result<PreferencesConfig, AppError>;
    async fn get_app_strings(&self, lang: Language) -> Result<HashMap<String, String>, AppError>;
}

#[derive(Clone, Default)]
pub struct InMemoryConfigRepository;

impl InMemoryConfigRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ConfigRepository for InMemoryConfigRepository {
    async fn get_preferences_config(&self, lang: Language) -> Result<PreferencesConfig, AppError> {
        match lang {
            Language::Hi => Ok(PreferencesConfig {
                title: "आपकी भोजन प्राथमिकताएँ".to_string(),
                subtitle: "आप जो पकाना और खाना पसंद करते हैं उसके आधार पर वैयक्तिकृत सुझाव पाएं।".to_string(),
                save_button_text: "प्राथमिकताएं सहेजें".to_string(),
                groups: vec![
                    PreferenceGroupConfig {
                        id: "cuisines".to_string(),
                        title: "व्यंजन शैलियाँ (Cuisines)".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "North Indian".to_string(), label: "उत्तर भारतीय".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/706/706164.png".to_string()) },
                            PreferenceOptionItem { id: "Maharashtrian".to_string(), label: "महाराष्ट्रीयन".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3075/3075977.png".to_string()) },
                            PreferenceOptionItem { id: "Italian".to_string(), label: "इटैलियन".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1404/1404945.png".to_string()) },
                            PreferenceOptionItem { id: "South Indian".to_string(), label: "दक्षिण भारतीय".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Pan-Asian".to_string(), label: "पैन-एशियाई".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Mediterranean".to_string(), label: "भूमध्यसागरीय".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3014/3014524.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "dietary".to_string(),
                        title: "आहार संबंधी विकल्प (Dietary)".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "Vegetarian".to_string(), label: "शाकाहारी (Veg)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/723/723633.png".to_string()) },
                            PreferenceOptionItem { id: "Vegan".to_string(), label: "वीगन (Vegan)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1415/1415433.png".to_string()) },
                            PreferenceOptionItem { id: "Jain".to_string(), label: "जैन (Jain)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3233/3233483.png".to_string()) },
                            PreferenceOptionItem { id: "Eggetarian".to_string(), label: "अंडा युक्त".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/837/837560.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "spice".to_string(),
                        title: "तीखापन पसंद (Spice Level)".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Mild".to_string(), label: "हल्का (Mild)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Medium".to_string(), label: "मध्यम (Medium)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Spicy".to_string(), label: "तीखा (Spicy)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Extra Hot".to_string(), label: "अत्यधिक तीखा (Extra Hot)".to_string(), icon_url: None },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "sweetness".to_string(),
                        title: "मिठास पसंद (Sweetness Level)".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Low".to_string(), label: "कम (Low)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Balanced".to_string(), label: "संतुलित (Balanced)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Sweet".to_string(), label: "मीठा (Sweet)".to_string(), icon_url: None },
                        ],
                    },
                ],
            }),
            Language::Ar => Ok(PreferencesConfig {
                title: "تفضيلاتك الغذائية".to_string(),
                subtitle: "اكتشف أشهى الوصفات المصممة خصيصاً لذوقك الشخصي.".to_string(),
                save_button_text: "حفظ التفضيلات".to_string(),
                groups: vec![
                    PreferenceGroupConfig {
                        id: "cuisines".to_string(),
                        title: "المطابخ المفضلة (Cuisines)".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "North Indian".to_string(), label: "هندي شمالي".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/706/706164.png".to_string()) },
                            PreferenceOptionItem { id: "Maharashtrian".to_string(), label: "ماهاراشتري".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3075/3075977.png".to_string()) },
                            PreferenceOptionItem { id: "Italian".to_string(), label: "إيطالي".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1404/1404945.png".to_string()) },
                            PreferenceOptionItem { id: "South Indian".to_string(), label: "هندي جنوبي".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Pan-Asian".to_string(), label: "آسيوي".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Mediterranean".to_string(), label: "متوسطي".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3014/3014524.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "dietary".to_string(),
                        title: "الخيارات الغذائية (Dietary)".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "Vegetarian".to_string(), label: "نباتي (Veg)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/723/723633.png".to_string()) },
                            PreferenceOptionItem { id: "Vegan".to_string(), label: "نباتي صرف (Vegan)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1415/1415433.png".to_string()) },
                            PreferenceOptionItem { id: "Jain".to_string(), label: "جاين (Jain)".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3233/3233483.png".to_string()) },
                            PreferenceOptionItem { id: "Eggetarian".to_string(), label: "مع بيض".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/837/837560.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "spice".to_string(),
                        title: "درجة الحرارة والتوابل (Spice Level)".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Mild".to_string(), label: "خفيف (Mild)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Medium".to_string(), label: "متوسط (Medium)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Spicy".to_string(), label: "حار (Spicy)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Extra Hot".to_string(), label: "حار جداً (Extra Hot)".to_string(), icon_url: None },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "sweetness".to_string(),
                        title: "درجة الحلاوة (Sweetness Level)".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Low".to_string(), label: "قليل السكر (Low)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Balanced".to_string(), label: "معتدل (Balanced)".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Sweet".to_string(), label: "حلو (Sweet)".to_string(), icon_url: None },
                        ],
                    },
                ],
            }),
            Language::En => Ok(PreferencesConfig {
                title: "Your Food Preferences".to_string(),
                subtitle: "Personalize recommendations based on what you love to cook and eat.".to_string(),
                save_button_text: "Save Preferences".to_string(),
                groups: vec![
                    PreferenceGroupConfig {
                        id: "cuisines".to_string(),
                        title: "Cuisines".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "North Indian".to_string(), label: "North Indian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/706/706164.png".to_string()) },
                            PreferenceOptionItem { id: "Maharashtrian".to_string(), label: "Maharashtrian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3075/3075977.png".to_string()) },
                            PreferenceOptionItem { id: "Italian".to_string(), label: "Italian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1404/1404945.png".to_string()) },
                            PreferenceOptionItem { id: "South Indian".to_string(), label: "South Indian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Pan-Asian".to_string(), label: "Pan-Asian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/2276/2276931.png".to_string()) },
                            PreferenceOptionItem { id: "Mediterranean".to_string(), label: "Mediterranean".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3014/3014524.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "dietary".to_string(),
                        title: "Dietary Preferences".to_string(),
                        is_multi_select: true,
                        options: vec![
                            PreferenceOptionItem { id: "Vegetarian".to_string(), label: "Vegetarian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/723/723633.png".to_string()) },
                            PreferenceOptionItem { id: "Vegan".to_string(), label: "Vegan".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/1415/1415433.png".to_string()) },
                            PreferenceOptionItem { id: "Jain".to_string(), label: "Jain".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/3233/3233483.png".to_string()) },
                            PreferenceOptionItem { id: "Eggetarian".to_string(), label: "Eggetarian".to_string(), icon_url: Some("https://cdn-icons-png.flaticon.com/512/837/837560.png".to_string()) },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "spice".to_string(),
                        title: "Spice Preference".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Mild".to_string(), label: "Mild".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Medium".to_string(), label: "Medium".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Spicy".to_string(), label: "Spicy".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Extra Hot".to_string(), label: "Extra Hot".to_string(), icon_url: None },
                        ],
                    },
                    PreferenceGroupConfig {
                        id: "sweetness".to_string(),
                        title: "Sweetness Preference".to_string(),
                        is_multi_select: false,
                        options: vec![
                            PreferenceOptionItem { id: "Low".to_string(), label: "Low".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Balanced".to_string(), label: "Balanced".to_string(), icon_url: None },
                            PreferenceOptionItem { id: "Sweet".to_string(), label: "Sweet".to_string(), icon_url: None },
                        ],
                    },
                ],
            }),
        }
    }

    async fn get_app_strings(&self, lang: Language) -> Result<HashMap<String, String>, AppError> {
        let mut map = HashMap::new();
        match lang {
            Language::Hi => {
                map.insert("appName".to_string(), "कुक्की".to_string());
                map.insert("welcomeTitle".to_string(), "स्वादिष्ट व्यंजन खोजें".to_string());
                map.insert("welcomeSubtitle".to_string(), "वास्तविक समय भाग, ऑडियो और प्रामाणिक रहस्यों के साथ कदम-दर-कदम खाना बनाएं।".to_string());
                map.insert("searchHint".to_string(), "व्यंजन, सामग्री, भोजन शैली खोजें...".to_string());
                map.insert("savePreferences".to_string(), "प्राथमिकताएं सहेजें".to_string());
                map.insert("profile".to_string(), "प्रोफ़ाइल".to_string());
                map.insert("favourites".to_string(), "पसंदीदा".to_string());
                map.insert("settings".to_string(), "सेटिंग्स".to_string());
                map.insert("logout".to_string(), "लॉग आउट".to_string());
            }
            Language::Ar => {
                map.insert("appName".to_string(), "كوكي".to_string());
                map.insert("welcomeTitle".to_string(), "اكتشف أشهى الوصفات".to_string());
                map.insert("welcomeSubtitle".to_string(), "اطبخ خطوة بخطوة مع توجيه صوتي وتعديل الحصص تلقائياً.".to_string());
                map.insert("searchHint".to_string(), "ابحث عن أطباق ومكونات...".to_string());
                map.insert("savePreferences".to_string(), "حفظ التفضيلات".to_string());
                map.insert("profile".to_string(), "الملف الشخصي".to_string());
                map.insert("favourites".to_string(), "المفضلة".to_string());
                map.insert("settings".to_string(), "الإعدادات".to_string());
                map.insert("logout".to_string(), "تسجيل الخروج".to_string());
            }
            Language::En => {
                map.insert("appName".to_string(), "Cukkie".to_string());
                map.insert("welcomeTitle".to_string(), "Discover Delicious Recipes".to_string());
                map.insert("welcomeSubtitle".to_string(), "Cook step-by-step with real-time portions, audio narration, and curated culinary secrets.".to_string());
                map.insert("searchHint".to_string(), "Search dishes, ingredients, cuisines...".to_string());
                map.insert("savePreferences".to_string(), "Save Preferences".to_string());
                map.insert("profile".to_string(), "Profile".to_string());
                map.insert("favourites".to_string(), "Favourites".to_string());
                map.insert("settings".to_string(), "Settings".to_string());
                map.insert("logout".to_string(), "Log Out".to_string());
            }
        }
        Ok(map)
    }
}
