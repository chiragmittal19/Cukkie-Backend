use crate::domain::*;
use common::Language;
use serde_json::json;

pub struct MockData;

impl MockData {
    pub fn get_header_title(lang: Language) -> &'static str {
        match lang {
            Language::Hi => "कुक्की रेसिपीज़",
            Language::Ar => "وصفات كوكي",
            Language::En => "Cukkie Recipes",
        }
    }

    pub fn get_header_subtitle(lang: Language) -> &'static str {
        match lang {
            Language::Hi => "अपनी पसंद और स्वाद के अनुसार प्रामाणिक व्यंजन खोजें",
            Language::Ar => "استكشف أشهى الأطباق الأصيلة المصممة حسب رغباتك",
            Language::En => "Explore authentic dishes tailored to your cravings",
        }
    }

    pub fn get_search_hint(lang: Language) -> &'static str {
        match lang {
            Language::Hi => "व्यंजन, सामग्री, भोजन शैली खोजें...",
            Language::Ar => "ابحث عن أطباق، مطابخ، مكونات...",
            Language::En => "Search dishes, cuisines, ingredients...",
        }
    }

    pub fn get_filter_groups(lang: Language) -> Vec<FilterGroup> {
        match lang {
            Language::Hi => vec![
                FilterGroup {
                    id: "cuisines".to_string(),
                    title: "व्यंजन शैलियाँ (Cuisines)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "North Indian".to_string(), label: "उत्तर भारतीय".to_string(), icon_url: None },
                        FilterOption { id: "Maharashtrian".to_string(), label: "महाराष्ट्रीयन".to_string(), icon_url: None },
                        FilterOption { id: "Italian".to_string(), label: "इटैलियन".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "dietary".to_string(),
                    title: "आहार संबंधी (Dietary)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Vegetarian".to_string(), label: "शाकाहारी".to_string(), icon_url: None },
                        FilterOption { id: "Vegan".to_string(), label: "वीगन".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "taste".to_string(),
                    title: "स्वाद प्रोफाइल (Taste Profile)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Spicy".to_string(), label: "तीखा".to_string(), icon_url: None },
                        FilterOption { id: "Sweet".to_string(), label: "मीठा".to_string(), icon_url: None },
                        FilterOption { id: "Savory".to_string(), label: "चटपटा/स्वादिष्ट".to_string(), icon_url: None },
                    ],
                },
            ],
            Language::Ar => vec![
                FilterGroup {
                    id: "cuisines".to_string(),
                    title: "المطابخ (Cuisines)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "North Indian".to_string(), label: "هندي شمالي".to_string(), icon_url: None },
                        FilterOption { id: "Maharashtrian".to_string(), label: "ماهاراشتري".to_string(), icon_url: None },
                        FilterOption { id: "Italian".to_string(), label: "إيطالي".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "dietary".to_string(),
                    title: "الخيارات الغذائية (Dietary)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Vegetarian".to_string(), label: "نباتي".to_string(), icon_url: None },
                        FilterOption { id: "Vegan".to_string(), label: "نباتي صرف".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "taste".to_string(),
                    title: "النكهات (Taste Profile)".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Spicy".to_string(), label: "حار".to_string(), icon_url: None },
                        FilterOption { id: "Sweet".to_string(), label: "حلو".to_string(), icon_url: None },
                        FilterOption { id: "Savory".to_string(), label: "مالح/لذيذ".to_string(), icon_url: None },
                    ],
                },
            ],
            Language::En => vec![
                FilterGroup {
                    id: "cuisines".to_string(),
                    title: "Cuisines".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "North Indian".to_string(), label: "North Indian".to_string(), icon_url: None },
                        FilterOption { id: "Maharashtrian".to_string(), label: "Maharashtrian".to_string(), icon_url: None },
                        FilterOption { id: "Italian".to_string(), label: "Italian".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "dietary".to_string(),
                    title: "Dietary Preferences".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Vegetarian".to_string(), label: "Vegetarian".to_string(), icon_url: None },
                        FilterOption { id: "Vegan".to_string(), label: "Vegan".to_string(), icon_url: None },
                    ],
                },
                FilterGroup {
                    id: "taste".to_string(),
                    title: "Taste Profile".to_string(),
                    is_multi_select: true,
                    options: vec![
                        FilterOption { id: "Spicy".to_string(), label: "Spicy".to_string(), icon_url: None },
                        FilterOption { id: "Sweet".to_string(), label: "Sweet".to_string(), icon_url: None },
                        FilterOption { id: "Savory".to_string(), label: "Savory".to_string(), icon_url: None },
                    ],
                },
            ],
        }
    }

    pub fn get_sorters(lang: Language) -> Vec<SortOption> {
        match lang {
            Language::Hi => vec![
                SortOption { id: "rating_desc".to_string(), label: "सर्वोच्च रेटिंग (Highest Rated)".to_string(), icon_url: None, is_default: true },
                SortOption { id: "prep_time_asc".to_string(), label: "कम तैयारी समय (Quickest)".to_string(), icon_url: None, is_default: false },
                SortOption { id: "calories_asc".to_string(), label: "कम कैलोरी (Low Calorie)".to_string(), icon_url: None, is_default: false },
            ],
            Language::Ar => vec![
                SortOption { id: "rating_desc".to_string(), label: "الأعلى تقييماً".to_string(), icon_url: None, is_default: true },
                SortOption { id: "prep_time_asc".to_string(), label: "الأسرع تحضيراً".to_string(), icon_url: None, is_default: false },
                SortOption { id: "calories_asc".to_string(), label: "سعرات أقل".to_string(), icon_url: None, is_default: false },
            ],
            Language::En => vec![
                SortOption { id: "rating_desc".to_string(), label: "Highest Rated".to_string(), icon_url: None, is_default: true },
                SortOption { id: "prep_time_asc".to_string(), label: "Prep Time (Shortest)".to_string(), icon_url: None, is_default: false },
                SortOption { id: "calories_asc".to_string(), label: "Calories (Lowest)".to_string(), icon_url: None, is_default: false },
            ],
        }
    }

    pub fn get_drawer_items(lang: Language, is_logged_in: bool) -> Vec<DrawerItem> {
        let (prof_t, fav_t, set_t, auth_t) = match lang {
            Language::Hi => ("प्रोफ़ाइल एवं प्राथमिकताएं", "पसंदीदा व्यंजन", "सेटिंग्स", if is_logged_in { "लॉग आउट" } else { "लॉग इन" }),
            Language::Ar => ("الملف الشخصي والتفضيلات", "الأطباق المفضلة", "الإعدادات", if is_logged_in { "تسجيل الخروج" } else { "تسجيل الدخول" }),
            Language::En => ("Profile & Preferences", "Favourites", "Settings", if is_logged_in { "Log Out" } else { "Log In" }),
        };

        vec![
            DrawerItem { id: "profile".to_string(), title: prof_t.to_string(), subtitle: None, icon_url: None, route: Some("/profile".to_string()), requires_auth: true },
            DrawerItem { id: "favourites".to_string(), title: fav_t.to_string(), subtitle: None, icon_url: None, route: Some("/favourites".to_string()), requires_auth: false },
            DrawerItem { id: "settings".to_string(), title: set_t.to_string(), subtitle: None, icon_url: None, route: Some("/settings".to_string()), requires_auth: false },
            DrawerItem { id: if is_logged_in { "logout" } else { "login" }.to_string(), title: auth_t.to_string(), subtitle: None, icon_url: None, route: Some(if is_logged_in { "/logout" } else { "/login" }.to_string()), requires_auth: false },
        ]
    }

    pub fn get_all_dishes(lang: Language) -> Vec<DishSummary> {
        match lang {
            Language::Hi => vec![
                DishSummary {
                    id: "dish_pbm".to_string(),
                    name: "पनीर बटर मसाला (Paneer Butter Masala)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1631452180519-c014fe946bc7?w=800".to_string(),
                    taste_profiles: vec!["मलाईदार".to_string(), "समृद्ध".to_string(), "हल्का तीखा".to_string()],
                    prep_time_minutes: 35,
                    calories_kcal: 380,
                    protein_grams: 14,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_misal".to_string(),
                    name: "कोल्हापुरी मिसल पाव (Kolhapuri Misal Pav)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1601050690597-df0568f70950?w=800".to_string(),
                    taste_profiles: vec!["तीखा".to_string(), "चटपटा".to_string(), "मसालेदार".to_string()],
                    prep_time_minutes: 30,
                    calories_kcal: 420,
                    protein_grams: 16,
                    is_favourite: false,
                    cuisine: "Maharashtrian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_paratha".to_string(),
                    name: "अमृतसरी आलू पराठा (Amritsari Aloo Paratha)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1626074353765-517a681e40be?w=800".to_string(),
                    taste_profiles: vec!["स्वादिष्ट".to_string(), "कुरकुरा".to_string(), "मक्खनयुक्त".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 310,
                    protein_grams: 7,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.7,
                },
                DishSummary {
                    id: "dish_halwa".to_string(),
                    name: "शाही गाजर का हलवा (Gajar Ka Halwa)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1599488615731-7e5c2823ff28?w=800".to_string(),
                    taste_profiles: vec!["मीठा".to_string(), "समृद्ध".to_string(), "इलायची सुगंध".to_string()],
                    prep_time_minutes: 45,
                    calories_kcal: 340,
                    protein_grams: 6,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_pizza".to_string(),
                    name: "क्लासिक मार्गेरीटा पिज्जा (Margherita Pizza)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?w=800".to_string(),
                    taste_profiles: vec!["स्वादिष्ट".to_string(), "चीज़ी".to_string(), "हर्बल".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 360,
                    protein_grams: 12,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_arrabbiata".to_string(),
                    name: "पेने ऑल'अराब्बियाटा (Penne all'Arrabbiata)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1551183053-bf91a1d81141?w=800".to_string(),
                    taste_profiles: vec!["तीखा".to_string(), "टमाटर खट्टा".to_string(), "लहसुनी".to_string()],
                    prep_time_minutes: 20,
                    calories_kcal: 290,
                    protein_grams: 9,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.6,
                },
            ],
            Language::Ar => vec![
                DishSummary {
                    id: "dish_pbm".to_string(),
                    name: "بانير بتر مسالا (Paneer Butter Masala)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1631452180519-c014fe946bc7?w=800".to_string(),
                    taste_profiles: vec!["كريمي".to_string(), "غني".to_string(), "بهارات خفيفة".to_string()],
                    prep_time_minutes: 35,
                    calories_kcal: 380,
                    protein_grams: 14,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_misal".to_string(),
                    name: "ميسال باو كولهابوري (Misal Pav)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1601050690597-df0568f70950?w=800".to_string(),
                    taste_profiles: vec!["حار".to_string(), "مقرمش".to_string(), "متبل".to_string()],
                    prep_time_minutes: 30,
                    calories_kcal: 420,
                    protein_grams: 16,
                    is_favourite: false,
                    cuisine: "Maharashtrian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_paratha".to_string(),
                    name: "باراثا البطاطس (Aloo Paratha)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1626074353765-517a681e40be?w=800".to_string(),
                    taste_profiles: vec!["لذيذ".to_string(), "مقرمش".to_string(), "زبدة".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 310,
                    protein_grams: 7,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.7,
                },
                DishSummary {
                    id: "dish_halwa".to_string(),
                    name: "حلوى الجزر الهندية (Gajar Ka Halwa)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1599488615731-7e5c2823ff28?w=800".to_string(),
                    taste_profiles: vec!["حلو".to_string(), "هيل".to_string(), "غني".to_string()],
                    prep_time_minutes: 45,
                    calories_kcal: 340,
                    protein_grams: 6,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_pizza".to_string(),
                    name: "بيتزا مارغريتا كلاسيكية (Margherita Pizza)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?w=800".to_string(),
                    taste_profiles: vec!["جبني".to_string(), "أعشاب".to_string(), "لذيذ".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 360,
                    protein_grams: 12,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_arrabbiata".to_string(),
                    name: "بيني أرابياتا (Penne all'Arrabbiata)".to_string(),
                    image_url: "https://images.unsplash.com/photo-1551183053-bf91a1d81141?w=800".to_string(),
                    taste_profiles: vec!["حار".to_string(), "طماطم طازجة".to_string(), "ثوم".to_string()],
                    prep_time_minutes: 20,
                    calories_kcal: 290,
                    protein_grams: 9,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.6,
                },
            ],
            Language::En => vec![
                DishSummary {
                    id: "dish_pbm".to_string(),
                    name: "Paneer Butter Masala".to_string(),
                    image_url: "https://images.unsplash.com/photo-1631452180519-c014fe946bc7?w=800".to_string(),
                    taste_profiles: vec!["Creamy".to_string(), "Rich".to_string(), "Mild Spicy".to_string()],
                    prep_time_minutes: 35,
                    calories_kcal: 380,
                    protein_grams: 14,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_misal".to_string(),
                    name: "Kolhapuri Misal Pav".to_string(),
                    image_url: "https://images.unsplash.com/photo-1601050690597-df0568f70950?w=800".to_string(),
                    taste_profiles: vec!["Spicy".to_string(), "Tangy".to_string(), "Savory".to_string()],
                    prep_time_minutes: 30,
                    calories_kcal: 420,
                    protein_grams: 16,
                    is_favourite: false,
                    cuisine: "Maharashtrian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_paratha".to_string(),
                    name: "Amritsari Aloo Paratha".to_string(),
                    image_url: "https://images.unsplash.com/photo-1626074353765-517a681e40be?w=800".to_string(),
                    taste_profiles: vec!["Savory".to_string(), "Crispy".to_string(), "Buttery".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 310,
                    protein_grams: 7,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.7,
                },
                DishSummary {
                    id: "dish_halwa".to_string(),
                    name: "Shahi Gajar Ka Halwa".to_string(),
                    image_url: "https://images.unsplash.com/photo-1599488615731-7e5c2823ff28?w=800".to_string(),
                    taste_profiles: vec!["Sweet".to_string(), "Rich".to_string(), "Cardamom Infused".to_string()],
                    prep_time_minutes: 45,
                    calories_kcal: 340,
                    protein_grams: 6,
                    is_favourite: false,
                    cuisine: "North Indian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.9,
                },
                DishSummary {
                    id: "dish_pizza".to_string(),
                    name: "Classic Margherita Pizza".to_string(),
                    image_url: "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?w=800".to_string(),
                    taste_profiles: vec!["Savory".to_string(), "Cheesy".to_string(), "Herbal".to_string()],
                    prep_time_minutes: 25,
                    calories_kcal: 360,
                    protein_grams: 12,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegetarian".to_string(),
                    rating: 4.8,
                },
                DishSummary {
                    id: "dish_arrabbiata".to_string(),
                    name: "Penne all'Arrabbiata".to_string(),
                    image_url: "https://images.unsplash.com/photo-1551183053-bf91a1d81141?w=800".to_string(),
                    taste_profiles: vec!["Spicy".to_string(), "Tangy".to_string(), "Garlicky".to_string()],
                    prep_time_minutes: 20,
                    calories_kcal: 290,
                    protein_grams: 9,
                    is_favourite: false,
                    cuisine: "Italian".to_string(),
                    dietary_type: "Vegan".to_string(),
                    rating: 4.6,
                },
            ],
        }
    }

    pub fn get_dish_detail(dish_id: &str, lang: Language) -> Option<DishDetail> {
        let (name, desc, hist) = match (dish_id, lang) {
            ("dish_pbm", Language::Hi) => (
                "पनीर बटर मसाला",
                "रेशमी टमाटर की ग्रेवी में पकाए गए ताज़ा पनीर क्यूब्स।",
                "मुगलई और पंजाबी पाक परंपराओं का अद्भुत मेल।"
            ),
            ("dish_pbm", Language::Ar) => (
                "بانير بتر مسالا",
                "مكعبات الجبن الهندي الطري في مرق الطماطم الغني بالزبدة والكريمة.",
                "أحد أشهر أطباق المطبخ الهندي الشمالي الكلاسيكي."
            ),
            ("dish_pbm", _) => (
                "Paneer Butter Masala",
                "Soft cottage cheese cubes enveloped in a velvety tomato-cashew reduction infused with fenugreek.",
                "Born in Delhi post-independence, bringing royal butter gravy to cottage cheese."
            ),
            ("dish_misal", _) => (
                "Kolhapuri Misal Pav",
                "Sprouted moth beans cooked in a fiery kat gravy topped with farsan and onions, served with ladi pav.",
                "Iconic Maharashtrian breakfast renowned for authentic spice and rustic aroma."
            ),
            ("dish_paratha", _) => (
                "Amritsari Aloo Paratha",
                "Flaky, golden whole wheat flatbread stuffed with spiced mashed potatoes.",
                "Heritage breakfast staple of Punjab cooked on cast-iron tawas."
            ),
            ("dish_halwa", _) => (
                "Shahi Gajar Ka Halwa",
                "Slow-simmered winter red carrots with whole milk, ghee, and golden nuts.",
                "Celebrated Mughal winter confection perfected in royal kitchens."
            ),
            ("dish_pizza", _) => (
                "Classic Margherita Pizza",
                "Neapolitan crust topped with crushed San Marzano tomatoes, fresh mozzarella, and basil leaves.",
                "Created in Naples in 1889 to honor Queen Margherita of Savoy."
            ),
            ("dish_arrabbiata", _) => (
                "Penne all'Arrabbiata",
                "Al dente penne tossed in fiery garlic, tomato, and red pepper flakes.",
                "Roman classic whose name literally translates to 'angry pasta' due to chili heat."
            ),
            _ => return None,
        };

        let portion = if dish_id == "dish_paratha" {
            PortionDefinition {
                unit: "parathas".to_string(),
                default_amount: 2.0,
                step_increment: 1.0,
                min_amount: 1.0,
                max_amount: 8.0,
            }
        } else if dish_id == "dish_halwa" {
            PortionDefinition {
                unit: "grams".to_string(),
                default_amount: 250.0,
                step_increment: 50.0,
                min_amount: 100.0,
                max_amount: 1000.0,
            }
        } else {
            PortionDefinition {
                unit: "servings".to_string(),
                default_amount: 2.0,
                step_increment: 1.0,
                min_amount: 1.0,
                max_amount: 10.0,
            }
        };

        let ingredients = vec![
            IngredientItem { id: "ing_1".to_string(), name: "Main ingredient".to_string(), base_quantity: 200.0, unit: "g".to_string(), is_optional: false, image_url: None },
            IngredientItem { id: "ing_2".to_string(), name: "Spices & aromatics".to_string(), base_quantity: 1.5, unit: "tbsp".to_string(), is_optional: false, image_url: None },
            IngredientItem { id: "ing_3".to_string(), name: "Sauce / gravy base".to_string(), base_quantity: 1.0, unit: "cup".to_string(), is_optional: false, image_url: None },
            IngredientItem { id: "ing_4".to_string(), name: "Garnish (Fresh herbs)".to_string(), base_quantity: 2.0, unit: "sprigs".to_string(), is_optional: true, image_url: None },
        ];

        let steps = vec![
            RecipeStepItem {
                index: 0,
                heading: "Description & Story".to_string(),
                instruction_text: desc.to_string(),
                audio_narration_text: format!("Welcome to cooking {}. {}", name, desc),
                todos: vec![],
                image_url: None,
            },
            RecipeStepItem {
                index: 1,
                heading: "Portion Calibration".to_string(),
                instruction_text: "Adjust your portion size. All ingredient amounts will scale proportionately.".to_string(),
                audio_narration_text: "Choose the portion size you wish to prepare.".to_string(),
                todos: vec![],
                image_url: None,
            },
            RecipeStepItem {
                index: 2,
                heading: "Mise en Place (Prep)".to_string(),
                instruction_text: "Gather and measure all ingredients before beginning heat application.".to_string(),
                audio_narration_text: "Assemble all spices, produce, and cooking equipment on your counter.".to_string(),
                todos: vec!["Chop aromatics".to_string(), "Measure ground spices".to_string(), "Prep main ingredient".to_string()],
                image_url: None,
            },
            RecipeStepItem {
                index: 3,
                heading: "Cooking & Simmering".to_string(),
                instruction_text: "Heat pan, temper aromatics, add base sauce and cook until fragrant.".to_string(),
                audio_narration_text: "Gently simmer the mixture until flavors meld seamlessly.".to_string(),
                todos: vec!["Sauté aromatics until golden".to_string(), "Add spices and cook for 1 minute".to_string(), "Simmer sauce until oil separates".to_string()],
                image_url: None,
            },
            RecipeStepItem {
                index: 4,
                heading: "Tasting & Feedback".to_string(),
                instruction_text: "Plate your culinary masterpiece and share your review.".to_string(),
                audio_narration_text: "Congratulations! Rate your dish and enjoy your meal.".to_string(),
                todos: vec![],
                image_url: None,
            },
        ];

        let feedback_form_fields = vec![
            DynamicFormField {
                id: "rating".to_string(),
                title: "How did it taste?".to_string(),
                subtitle: Some("Rate your overall culinary result".to_string()),
                field_type: "rating".to_string(),
                is_required: true,
                options: None,
                placeholder: None,
                initial_value: Some(json!(5)),
            },
            DynamicFormField {
                id: "review".to_string(),
                title: "Chef's Notes & Experience".to_string(),
                subtitle: Some("Any tweaks or tips for next time?".to_string()),
                field_type: "text".to_string(),
                is_required: false,
                options: None,
                placeholder: Some("Write your review...".to_string()),
                initial_value: None,
            },
            DynamicFormField {
                id: "photo".to_string(),
                title: "Upload Dish Photo".to_string(),
                subtitle: Some("Show off your culinary creation".to_string()),
                field_type: "image_upload".to_string(),
                is_required: false,
                options: None,
                placeholder: None,
                initial_value: None,
            },
        ];

        Some(DishDetail {
            id: dish_id.to_string(),
            name: name.to_string(),
            image_url: "https://images.unsplash.com/photo-1631452180519-c014fe946bc7?w=800".to_string(),
            share_url: format!("https://cukkie.com/recipe/{}", dish_id),
            description: desc.to_string(),
            history: hist.to_string(),
            nutritional_breakdown: json!({
                "calories": "380 kcal",
                "carbs": "28g",
                "fat": "24g",
                "protein": "14g",
                "fiber": "4g"
            }),
            portion_definition: portion,
            ingredients,
            steps,
            feedback_form_fields,
            is_favourite: false,
        })
    }
}
