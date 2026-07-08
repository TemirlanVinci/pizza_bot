CREATE TABLE categories (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO categories VALUES (1, 'Завтраки');
INSERT INTO categories VALUES (2, 'Детское меню');
INSERT INTO categories VALUES (3, 'Пицца 30см');
INSERT INTO categories VALUES (4, 'Пицца 40см');
INSERT INTO categories VALUES (5, 'Закуски');
INSERT INTO categories VALUES (6, 'Салаты');
INSERT INTO categories VALUES (7, 'Супы');
INSERT INTO categories VALUES (8, 'Горячие блюда');
INSERT INTO categories VALUES (9, 'Гарниры');
INSERT INTO categories VALUES (10, 'Соусы');
INSERT INTO categories VALUES (11, 'Роллы');
INSERT INTO categories VALUES (12, 'Сеты');
INSERT INTO categories VALUES (13, 'Безалкогольные коктейли');
INSERT INTO categories VALUES (14, 'Горячие напитки');
INSERT INTO categories VALUES (15, 'Десерты');
INSERT INTO categories VALUES (16, 'Хлеб');
INSERT INTO categories VALUES (17, 'Сигареты');
INSERT INTO categories VALUES (18, 'Холодные напитки');
INSERT INTO categories VALUES (19, 'Пиво');
INSERT INTO categories VALUES (20, 'Вино');
INSERT INTO categories VALUES (21, 'Элитные спиртные напитки');
INSERT INTO categories VALUES (22, 'Водка');


CREATE TABLE products (
    id          INTEGER PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id),
    name        TEXT NOT NULL,
    description TEXT,
    price       INTEGER NOT NULL,  
    weight      INTEGER NOT NULL,     
    image_url   TEXT NOT NULL
);

INSERT INTO products VALUES (
    1, 
    1, 
    'Блинный сет', 
    'КУЙМАК ЖЫЙНАГЫ Сыр креми жана лосось кошулган куймактар, эт менен куймак, классикалык куймак, Козу карын кошулган каймак соусунда бышырылган куймактар. Куурулган козу карындар, сырдуу соус жана каймак менен берилет БЛИННЫЙ СЕТ Блины с сырным кремом и лососем, блины с мясом, классические блины, Запеченные блины в сливочном соусе с грибами. Подается с жареными грибами, сырным соусом и сметаной',
    1498,
    700, 
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v357/Zavtraki/Blinnyii_set_gribyi/Medium.png?hash=1601255f2a33bd309c7feedc225a96ef'
 );

INSERT INTO products VALUES (
    2,
    1,
    'Блины с маком',
    'АПИЙИМ УРУГУ КОШУЛГАН КУЙМАК БЛИНЫ С МАКОМ ',
    188,
    130 ,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v355/Zavtraki/Blinyi_s_makom/Medium.png?hash=c34305bca4387ec47158fcb2d3116f16'
);

INSERT INTO products VALUES (
    3,
    1,
    'Блины с мясом ',
    'ЭТ МЕНЕН КУЙМАК БЛИНЫ С МЯСОМ Pancakes with meat.',
    388,
    160,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v63/Zavtraki/Blinyi_s_miasom/Medium.png?hash=727b52ce5db7a9ab358523e795a12b67'
);

INSERT INTO products VALUES (
    4,
    1,
    'Блины с творогом',
    'БЫШТАК МЕНЕН КУЙМАК БЛИНЫ С ТВОРОГОМ Pancakes with cottage cheese. ',
    258,
    170,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zavtraki/Blinyi_s_tvorogom/Medium.png?hash=bec2a171c9ce2ac169d99161538a7f8f'
);

INSERT INTO products VALUES (
    5,
    1,
    'Блины со сметаной ',
    'КАЙМАК МЕНЕН КУЙМАК БЛИНЫ СО СМЕТАНОЙ Pancakes with sour cream. ',
    168,
    120,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zavtraki/Blinyi_so_smetanoi/Medium.png?hash=f71e4b486d03b0926799d2ec9f7ddbb0'
);

INSERT INTO products VALUES (
    6,
    1,
    'Блины со шпинатом и фетой',
    'ШПИНАТ ЖАНА ФЕТА КОШУЛГАН КУЙМАКТАР Фетаки сыры жана шпинат кошулган, каймак майында куурулган куймактар БЛИНЫ СО ШПИНАТОМ И ФЕТОЙ Блины с сыром фетаки, шпинатом, поджаренные на сливочном масле ',
    278,
    140,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v355/Zavtraki/Blinyi_so_shpinatom_i_fetoi/Medium.png?hash=eec4af650abcc8bc4aa1e10dda65c67f'
);

INSERT INTO products VALUES (
    7,
    1,
    'Блины тирамису',
    'ТИРАМИСУ КУЙМАКТАРЫ Кофе жыттуу таттуу тирамису креми жана кытырак безе кошулган тирамису куймактары БЛИНЫ ТИРАМИСУ Блины тирамису со сладким кремом тирамису с ароматом кофе и хрустящим безе ',
    278,
    145,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v355/Zavtraki/Blinyi_tiramisu/Medium.png?hash=3a7ebeec72fb6acea26e71a1dd7dcb51'
);

INSERT INTO products VALUES (
    8,
    1,
    'Гречневая каша',
    'ГРЕЧКА БУЛАМЫГЫ ГРЕЧНЕВАЯ КАША ',
    178,
    320,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v224/Zavtraki/Grechnevaia_kasha/Medium.png?hash=3038eac4f82e169be75513802d280e6a'
);

INSERT INTO products VALUES (
    9,
    1,
    'Запеченные блины с грибами',
    'ЗАПЕЧЕННЫЕ БЛИНЫ В СЛИВОЧНОМ СОУСЕ Начинка : грибы КАЙМАКТУУ СОУСТА БЫШЫРЫЛГАН КУЙМАК Тандоо салмасы: козу карын',
    338,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v255/Erte_menenki_tamak/Zapechennyie_blinyi_v_slivoch_souse_gribyi/Medium.png?hash=18cdea060d92b3edc46b6a6073d194c5'
);

INSERT INTO products VALUES (
    10,
    1,
    'Запеченные блины с курицей',
    'ЗАПЕЧЕННЫЕ БЛИНЫ В СЛИВОЧНОМ СОУСЕ Начинка на выбор: курица КАЙМАКТУУ СОУСТА БЫШЫРЫЛГАН КУЙМАК Тандоо салмасы: тоок эти ',
    338,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v255/Erte_menenki_tamak/Zapechennyie_blinyi_v_slivoch_souse_kuritsa/Medium.png?hash=18cdea060d92b3edc46b6a6073d194c5'
);

INSERT INTO products VALUES (
    11,
    2,
    'Веселые осьминожки',
    'К???лд?? осьминожкалар Тойгузуучу бантик формасындагы макарон жана сосискалар жумшак каймак соусунда, к?к ч?пт?р менен. Веселые осьминожки Сытные макароны-бантики и сосиски в мягком сливочном соусе с зеленью.',
    218,
    160,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Veselyie_osminozhki/Medium.png?hash=da82df5f756d8056c7817017f8ab0ddd'
);

INSERT INTO products VALUES (
    12,
    2,
    'Котлетка"Микки"',
    '«Микки» котлети С?й?кт?? уй котлетинин оригиналдуу берилиши, абадай жумшак картошка пюреси, маслиналар жана к?к ч?пт?р менен. Котлетка «Микки» Оригинальная подача любимой говяжьей котлетки с воздушным картофельным пюре, маслинами и зеленью. ',
    328,
    180,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Kotletka_Mikki/Medium.png?hash=090883d82c70ec4f9d2b5735c7a7c9fa'
);

INSERT INTO products VALUES (
    13,
    2,
    'Мини бургер с фри',
    'Мини-бургер фри менен Классикалык бургер уй этинен жасалган котлет менен. Картошка фринин порциясы менен берилет. Мини-бургер с фри Классический бургер с говяжьей котлетой. Подается с порцией картофеля фри. ',
    388,
    220,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Mini_burger_s_fri/Medium.png?hash=4218ea658bc355c80dcb13e71cd107c7'
);

INSERT INTO products VALUES (
    14,
    2,
    'Пицца M&Ms',
    '«M&M’s» пиццасы Шоколад, сыр, маршмэллоу жана т?рк?н т?ст?? M&M’s дражелери кошулган таттуу пицца. Пицца «M&M’s» Сладкая пицца с шоколадом, сыром, маршмэллоу и яркими драже M&M’s. ',
    548,
    420,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Pitstsa_M_M_s/Medium.png?hash=ab3f42e83e250cdfdf544b872daea868'
);

INSERT INTO products VALUES (
    15,
    2,
    'Пицца "Сладкий тайник"',
    '«Таттуу жашыруун» пиццасы Сырдуу пицца! Назик кабыгынын ичинде эрип турган кара шоколад жана жумшак маршмэллоу жашырылган. Пицца «Сладкий тайник» Пицца с секретом! Под нежной корочкой скрывается тающий темный шоколад и воздушный маршмэллоу ',
    398,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Pitstsa_Sladkii_tainik/Medium.png?hash=6072d26b1a64e38ca777b18d72062b55'
);

INSERT INTO products VALUES (
    16,
    2,
    'Суп"Алфавит"с фрикадельками',
    '«Алфавит» шорпосу фрикаделькалар менен Уй этинен жасалган фрикаделькалар, жашылчалар жана тамга формасындагы макарон кошулган ?й шорпосу. Суп «Алфавит» с фрикадельками Домашний суп с говяжьими фрикадельками, овощами и макаронами-буквами.',
    188,
    350,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Sup_Alfavit_s_frikadelkami/Medium.png?hash=4ce6df6b0121c13ac57504fafa37eaf8'
);

INSERT INTO products VALUES (
    17,
    2,
    'Сырные палочки',
    'Сыр таякчалары Кытырак, назик моцарелладан жасалган таякчалар. Классикалык тар-тар соусу менен берилет. Сырные палочки Хрустящие палочки из нежной моцареллы. Подаются с классическим соусом тар-тар. ',
    308,
    150,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v373/Detskoe_meniu/Syirnyie_palochki/Medium.png?hash=a62a9cae2e4a58a23cd3a4e534e7ffff'
);

INSERT INTO products VALUES (
    18,
    3,
    '4 Сыра 30см ',
    '4 СЫР Пицца-соус, сырлар: моцарелла, эмменталь, дорблю, пармезан. 4 СЫРА Пицца-соус, сыры: моцарелла, эмменталь, дорблю, пармезан. 4 Cheese Pizza. Pizza sauce with mozzarella, emmental, dorblue, parmesan',
    818,
    550,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v45/Pitstsa_30sm/4_Syira_30sm/Medium.png?hash=f530c2fc1726d73286f6656fd56d59ae'
);

INSERT INTO products VALUES (
    19,
    3,
    'Барбекю 30см',
    'БАРБЕКЮ Барбекю соусу, тооктун сулп эти, колбаса, козу карын, томат, маринаддалган бадыра? менен пияз, жалбыз, жана ачуу калемпир. БАРБЕКЮ Сытная домашняя пицца с соусом барбекю, куриным филе, колбасками, грибами, томатами, маринованными огурчиками и луком с мятой и острым перчиком Hearty homemade pizza with barbecue sauce, chicken fillet, sausages, mushrooms, tomatoes, pickled onions and cucumbers. ',
    708,
    700,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Barbekiu_30sm/Medium.png?hash=b35d2b33c43f3083ad5154b1c8ab1859'
);

INSERT INTO products VALUES (
    20,
    3,
    'Вегетарианская 30см',
    'ВЕГЕТЕРИАНДАР YЧYН Пицца-соус, моцарелла сыры, помидор, ж?г?р?, зайтун, шампиньон, болгар калемпири, ч?п-чарлар, пияз. ВЕГЕТАРИАНСКАЯ Пицца-соус, сыр моцарелла, помидоры, кукуруза, маслины, шампиньоны, болгарский перец, зелень, лук. Vegetarian Pizza sauce, Mozzarella, greens, onion, champignon mushrooms, bell pepper, tomatoes, olives, corn ',
    628,
    600,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v32/Pitstsa_30sm/Vegetarianskaia_30sm/Medium.png?hash=5696140c19c5358cc13389a0122876fd'
);

INSERT INTO products VALUES (
    21,
    3,
    'Гасконская 30см',
    'ГАСКОНДУК Пицца-соус, моцарелла сыры, ышталган тоок, шампиньон, болгар калемпири, помидор, ж?г?р?, майонез, ч?п-чарлар. ГАСКОНСКАЯ Пицца-соус, сыр моцарелла, копчёная курица, шампиньоны, болгарский перец, помидоры, кукуруза, майонез, зелень. Gascon Pizza. Pizza sauce, mozzarella, smoked chicken, сhampignon mushrooms, bell peppers, tomatoes, corn, mayonnaise, parsley. ',
    758,
    740,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Gaskonskaia_30sm/Medium.png?hash=5ad10721beff448a1c86e59f30d3c2d9'
);

INSERT INTO products VALUES (
    22,
    3,
    'Деревенская 30см',
    'ЭЛЕТТИК Пицца-соус, моцарелла сыры, маринаддалган уй эти, ышталган тоок эти, шампиньон, сарымсак, пияз, ч?п-чарлар. ДЕРЕВЕНСКАЯ Пицца-соус, сыр моцарелла, маринованная говядина, копчёная курица, шампиньоны, чеснок, лук, зелень. Village Pizza. Pizza sauce, mozzarella, marinated beef, smoked duck breast, champignon mushrooms, garlic, onion, greens.',
    818,
    670,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Derevenskaia_30sm/Medium.png?hash=ed1e51a8a26eb31dfda14430d67fd2b6'
);

INSERT INTO products VALUES (
    23,
    3,
    'Домашняя 30см',
    '?ЙД?Г?Д?Й Пицца-соус, моцарелла сыры, ветчина, помидор, шампиньон, ч?п-чарлар. ДОМАШНЯЯ Пицца-соус, сыр моцарелла, ветчина Homemade Pizza. Pizza sauce, mozzarella, ham, tomatoes, champignon mushrooms, greens',
    588,
    580,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Domashniaia_30sm/Medium.png?hash=bd37cfba4ce5bc6ddd1ac72f061a3ba5'
);

INSERT INTO products VALUES (
    24,
    3,
    'Итальянка 30см ',
    'ИТАЛИЯЛЫК Пицца-соус, моцарелла сыры, каймактуу соуста вёшенки козу карыны менен уй эти, ч?п-чарлар. ИТАЛЬЯНКА Пицца-соус, сыр моцарелла, говядина с грибами вёшенками в сливочном соусе, зелень. . Italian Pizza. Pizza sauce, mozzarella, beef with oyster mushrooms in creamy sauce, greens. ',
    778,
    720,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Italianka_30sm/Medium.png?hash=20ad7fcd72e108089f2ca2fe79364e44'
);

INSERT INTO products VALUES (
    25,
    3,
    'Карри 30см',
    'КАРРИ Пицца-соус, моцарелла сыры, ачуу ?зг?ч?л?? соуста карри тоогу, кинза. КАРРИ Пицца-соус, сыр моцарелла, курица карри в остром фирменном соусе, кинза. Curry Pizza. Pizza sauce, mozzarella, curry chicken in spicy sauce, cilantro. ',
    778,
    760,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Karri_30sm/Medium.png?hash=ad9ccd83b74f2a2280988b57f9926449'
);

INSERT INTO products VALUES (
    26,
    3,
    'Классика 30см',
    'КЛАССИКА Пицца-соус, моцарелла сыры, ветчина, салями, шампиньон. КЛАССИКА Пицца-соус, сыр моцарелла, ветчина, салями, шампиньоны Classic Pizza. Pizza sauce, mozzarella, ham, salami, champignon mushrooms.',
    628,
    595,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Klassika_30sm/Medium.png?hash=7912b7354473adc68ed589d2f71d8da4'
);

INSERT INTO products VALUES (
    27,
    4,
    '4 Сыра 40см ',
    '4 СЫР Пицца-соус, сырлар: моцарелла, эмменталь, дорблю, пармезан. 4 СЫРА Пицца-соус, сыры: моцарелла, эмменталь, дорблю, пармезан. 4 Cheese Pizza. Pizza sauce with mozzarella, emmental, dorblue, parmesan',
    1158,
    910,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v45/Pitstsa_30sm/4_Syira_30sm/Medium.png?hash=f530c2fc1726d73286f6656fd56d59ae'
);

INSERT INTO products VALUES (
    28,
    4,
    'Барбекю 40см',
    'БАРБЕКЮ Барбекю соусу, тооктун сулп эти, колбаса, козу карын, томат, маринаддалган бадыра? менен пияз, жалбыз, жана ачуу калемпир. БАРБЕКЮ Сытная домашняя пицца с соусом барбекю, куриным филе, колбасками, грибами, томатами, маринованными огурчиками и луком с мятой и острым перчиком Hearty homemade pizza with barbecue sauce, chicken fillet, sausages, mushrooms, tomatoes, pickled onions and cucumbers. ',
    1128,
    1160,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Barbekiu_30sm/Medium.png?hash=b35d2b33c43f3083ad5154b1c8ab1859'
);

INSERT INTO products VALUES (
    29,
    4,
    'Вегетарианская 40см',
    'ВЕГЕТЕРИАНДАР YЧYН Пицца-соус, моцарелла сыры, помидор, ж?г?р?, зайтун, шампиньон, болгар калемпири, ч?п-чарлар, пияз. ВЕГЕТАРИАНСКАЯ Пицца-соус, сыр моцарелла, помидоры, кукуруза, маслины, шампиньоны, болгарский перец, зелень, лук. Vegetarian Pizza sauce, Mozzarella, greens, onion, champignon mushrooms, bell pepper, tomatoes, olives, corn ',
    1018,
    1040,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v32/Pitstsa_30sm/Vegetarianskaia_30sm/Medium.png?hash=5696140c19c5358cc13389a0122876fd'
);

INSERT INTO products VALUES (
    30,
    4,
    'Гасконская 40см',
    'ГАСКОНДУК Пицца-соус, моцарелла сыры, ышталган тоок, шампиньон, болгар калемпири, помидор, ж?г?р?, майонез, ч?п-чарлар. ГАСКОНСКАЯ Пицца-соус, сыр моцарелла, копчёная курица, шампиньоны, болгарский перец, помидоры, кукуруза, майонез, зелень. Gascon Pizza. Pizza sauce, mozzarella, smoked chicken, сhampignon mushrooms, bell peppers, tomatoes, corn, mayonnaise, parsley. ',
    1158,
    1310,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Gaskonskaia_30sm/Medium.png?hash=5ad10721beff448a1c86e59f30d3c2d9'
);

INSERT INTO products VALUES (
    31,
    4,
    'Деревенская 40см',
    'ЭЛЕТТИК Пицца-соус, моцарелла сыры, маринаддалган уй эти, ышталган тоок эти, шампиньон, сарымсак, пияз, ч?п-чарлар. ДЕРЕВЕНСКАЯ Пицца-соус, сыр моцарелла, маринованная говядина, копчёная курица, шампиньоны, чеснок, лук, зелень. Village Pizza. Pizza sauce, mozzarella, marinated beef, smoked duck breast, champignon mushrooms, garlic, onion, greens.',
    1288,
    1225,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Derevenskaia_30sm/Medium.png?hash=ed1e51a8a26eb31dfda14430d67fd2b6'
);

INSERT INTO products VALUES (
    32,
    4,
    'Домашняя 40см',
    '?ЙД?Г?Д?Й Пицца-соус, моцарелла сыры, ветчина, помидор, шампиньон, ч?п-чарлар. ДОМАШНЯЯ Пицца-соус, сыр моцарелла, ветчина Homemade Pizza. Pizza sauce, mozzarella, ham, tomatoes, champignon mushrooms, greens',
    888,
    990,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Domashniaia_30sm/Medium.png?hash=bd37cfba4ce5bc6ddd1ac72f061a3ba5'
);

INSERT INTO products VALUES (
    33,
    4,
    'Итальянка 40см ',
    'ИТАЛИЯЛЫК Пицца-соус, моцарелла сыры, каймактуу соуста вёшенки козу карыны менен уй эти, ч?п-чарлар. ИТАЛЬЯНКА Пицца-соус, сыр моцарелла, говядина с грибами вёшенками в сливочном соусе, зелень. . Italian Pizza. Pizza sauce, mozzarella, beef with oyster mushrooms in creamy sauce, greens. ',
    1288,
    1200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Italianka_30sm/Medium.png?hash=20ad7fcd72e108089f2ca2fe79364e44'
);

INSERT INTO products VALUES (
    153,
    4,
    'Карри 40см',
    'КАРРИ Пицца-соус, моцарелла сыры, ачуу ?зг?ч?л?? соуста карри тоогу, кинза. КАРРИ Пицца-соус, сыр моцарелла, курица карри в остром фирменном соусе, кинза. Curry Pizza. Pizza sauce, mozzarella, curry chicken in spicy sauce, cilantro. ',
    1168,
    1290,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Karri_30sm/Medium.png?hash=ad9ccd83b74f2a2280988b57f9926449'
);

INSERT INTO products VALUES (
    34,
    4,
    'Классика 40см',
    'КЛАССИКА Пицца-соус, моцарелла сыры, ветчина, салями, шампиньон. КЛАССИКА Пицца-соус, сыр моцарелла, ветчина, салями, шампиньоны Classic Pizza. Pizza sauce, mozzarella, ham, salami, champignon mushrooms.',
    1028,
    1065,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Pitstsa_30sm/Klassika_30sm/Medium.png?hash=7912b7354473adc68ed589d2f71d8da4'
);

INSERT INTO products VALUES (
    35,
    5,
    'Атлантика',
    'АТЛАНТИКА Азыраак туздалган Атлантика сельди, сууга бышкан карт?шк?, пияз, ч?п-чарлар, горчица. АТЛАНТИКА Атлантическая слабосолёная сельдь, отварной картофель, лук, зелень, горчица. Atlantic slightly salted herring, boiled potatoes, onions, herbs. ',
    458,
    330,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zakuski/Atlantika/Medium.png?hash=ae7197f1b66fa65b638ad8cf17858007'
);

INSERT INTO products VALUES (
    36,
    5,
    'Картофельные драники',
    'КАРТ?ШК? ДРАНИКИ Каймак менен берилет. КАРТОФЕЛЬНЫЕ ДРАНИКИ Подается со сметаной.. ',
    168,
    160,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v332/Zakuskalar/Kartofelnyie_draniki/Medium.png?hash=a2db64969c33653c1fe446d07cd56e07'
);

INSERT INTO products VALUES (
    37,
    5,
    'Крылышки барбекю',
    'БАРБЕКЮ КАНАТТАРЫ КРЫЛЫШКИ БАРБЕКЮ Barbecue chicken wings. ',
    628,
    270,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zakuski/Kryilyishki_barbekiu/Medium.png?hash=50c82fda67d0437db7c02116f5947588'
);

INSERT INTO products VALUES (
    38,
    5,
    'Крылышки под соусом терияки',
    'ТЕРИЯКИ СОУСУНДА КАНАТТАР КРЫЛЫШКИ ПОД СОУСОМ ТЕРИЯКИ Teriyaki chicken wings. ',
    628,
    270,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zakuski/Kryilyishki_pod_sousom_teriiaki/Medium.png?hash=3d977c634021ea7269c74d849ebd0957'
);

INSERT INTO products VALUES (
    39,
    5,
    'Наггетсы из курицы',
    'ТООК ЭТИНЕН НАГГЕТСТЕР Тартар ?зг?ч?л?? соусу менен берилет. НАГГЕТСЫ С КУРИЦЕЙ Подаются с фирменным соусом тартар. Chicken nuggets. Served with special sauce tartar. ',
    438,
    165,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Zakuski/Naggetsyi_iz_kuritsyi/Medium.png?hash=33af30fe977292fdd59cd4e76e6362ac'
);

INSERT INTO products VALUES (
    40,
    5,
    'Хрустящая шаурма с курицей',
    'ТООК ЭТИ КОШУЛГАН КЫТЫРАК ШАУРМА Лаваш, тооктун т?ш эти, капуста, бадыра?, помидор, кызыл пияз, сарымсак, кетчуп, майонез, горчица. ХРУСТЯЩАЯ ШАУРМА С КУРИЦЕЙ Лаваш, куриное филе, капуста, огурец, помидор, красный лук, чеснок, кетчуп, майонез, горчица ',
    288,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v332/Zakuskalar/Khrustiashchaia_shaurma_s_kuritsei/Medium.png?hash=8894fb69415955058f59c76c060f874e'
);

INSERT INTO products VALUES (
    41,
    5,
    'Хрустящая шаурма с острой говядиной',
    'АЧУУ УЙ ЭТИ КОШУЛГАН КЫТЫРАК ШАУРМА Лаваш, уй эти, Айсберг салаты, маринаддалган бадыра?, помидор, пияз, халапеньо калемпири, кетчуп, майонез. ХРУСТЯЩАЯ ШАУРМА С ОСТРОЙ ГОВЯДИНОЙ Лаваш, говядина, салат Айсберг, огурцы маринованные, помидор, лук, перец халапеньо, кетчуп, майонез ',
    328,
    260,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v332/Zakuskalar/Khrustiashchaia_shaurma_s_ostroi_goviadinoi/Medium.png?hash=2558e3ef93a7df5cf5481f3b9e990dfe'
);

INSERT INTO products VALUES (
    42,
    5,
    'Хрустящие куриные крылышки',
    'ТООКТУН КЫТЫРАК КАНАТТАРЫ ХРУСТЯЩИЕ КУРИНЫЕ КРЫЛЫШКИ Crispy chicken wings.',
    628,
    270,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v19/Zakuski/Khrustiashchie_kurinyie_kryilyishki/Medium.png?hash=de201bf4de991c1fc66bcdd0886fcd50'
);

INSERT INTO products VALUES (
    43,
    5,
    'Шаурма с курицей и картофелем пай ',
    'ТООК ЭТИ ЖАНА ПАЙ КАРТ?ШК?С? КОШУЛГАН ШАУРМА Лаваш, тооктун т?ш эти, корей сабиз, бадыра?, пай карт?шк?с?, майонез, кетчуп, кунжут. Халапеньо калемпири менен берилет. ШАУРМА С КУРИЦЕЙ И КАРТОФЕЛЕМ ПАЙ Лаваш, куринная грудка, морковь по-корейски, огурец, картофель пай, маойнез, кетчуп, кунжут. Подается с перцем халапеньо',
    348,
    270,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v219/Zakuski/Shaurma_s_kuritsei_i_kartofelem_pai/Medium.png?hash=d845449ec0fa6440379ef7202b4da55c'
);

INSERT INTO products VALUES (
    44,
    5,
    'Шаурма с соусом тар-тар',
    'ШАУРМА ТАР-ТАР ЧЫГЫ МЕНЕН Калама, тооктун т?ш эти, помидор, бадыра?, салат жалбырагы, Тар-тар чыгы, к?нж?т. ШАУРМА С СОУСОМ ТАР-ТАР Лаваш, куриная грудка, помидоры, огурцы, листья салата, соус Тар-тар, кунжут.',
    348,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v173/Zakuski/Shaurma_s_sousom_tar-tar/Medium.png?hash=abe9d16fd646becf1ea291a23a3fa7c5'
);


INSERT INTO products VALUES (
    45,
    6,
    'Брокколи по-китайски салат',
    'Брокколи, морковь, чеснок. КЫТАЙ БРОККОЛИСИ Брокколи, сабиз, сарымсак. Chinese broccoli salad. Broccoli, carrots, garlic.',
    348,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Salatyi/Brokkoli_po-kitaiski_salat/Medium.png?hash=c1472161d63a659f6d7105e744d20b37'
);

INSERT INTO products VALUES (
    46,
    6,
    'Весенний салат',
    'ЖАЗГЫ САЛАТ Редис, бадыра?, салат жалбырактары, шпинат, б?д?н? жумурткалары. Каймак менен берилет. ВЕСЕННИЙ САЛАТ Редис, огурец, листья салата, шпинат, яйца перепелиные. Заправлен сметаной.',
    268,
    220,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v332/Salads/Vesennii_salat/Medium.png?hash=41f75c58d5b7b6146bb397ec2dba092a'
);

INSERT INTO products VALUES (
    47,
    6,
    'Греческий салат',
    'Помидоры, болгарский перец, огурцы, сыр фетаки, микс листьев салата, маслины. Заправлен фирменным соусом. ГРЕК САЛАТЫ Помидор, болгар калемпири, бадыра?, фетаки сыры, салат жалбырактарынын аралашмасы, зайтун. Фирмалык чык менен аралаштырылат. Greek salad. Tomatoes, bell peppers, cucumbers, fetaki cheese, mix of lettuce leaves, olives. Served with branded sauce',
    468,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Salatyi/Grecheskii_salat/Medium.png?hash=3a42a135052a11d0f020484b66d2c982'
);

INSERT INTO products VALUES (
    48,
    6,
    'Кимчи',
    'Традиционное корейское блюдо из пекинской капусты. КИМЧИ Пекин капустасынан жасалган салттуу корей тамагы. ',
    178,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v219/Salatyi/Kimchi/Medium.png?hash=cc4174b9e9901833c9fbaeb752330153'
);

INSERT INTO products VALUES (
    49,
    6,
    'Ленивые язычки салат',
    'Баклажаны, помидоры, чеснок, зелень. Заправлены майонезом. ИЛЕНДИ ТИЛЧЕЛЕР САЛАТЫ Баклажан, помидор, сарымсак, ч?п-чарлар. Майонез менен аралаштырылат. Lazy tongues salad. Eggplant, tomatoes, garlic, greens. Seasoned with mayonnaise.',
    348,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Salatyi/Lenivyie_iazyichki_salat/Medium.png?hash=00388237d0efa428318545d81d9bd1ec'
);

INSERT INTO products VALUES (
    50,
    6,
    'Лесная поляна салат',
    ' Копченое филе курицы, маринованные огурцы, картофель, грибы вёшенки. Украшен зеленым луком. ТОКОЙ АЯНТЫ К?к пияз менен кооздолгон ышталган тоок этинин тилими, маринаддалган бадыра?, карт?шк?, вёшенки козу карыны. ',
    468,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Salattar/Lesnaia_poliana_salat/Medium.png?hash=016eb9c0fed7454ff94d8cd39c1a2a90'
);

INSERT INTO products VALUES (
    51,
    6,
    'Мидори Сарада салат',
    'Шпинат, листья салата, помидоры, огурцы, кинза, укроп. Заправлен фирменным соусом. МИДОРИ САРАДА Фирмалык чык менен аралаштырылган шпинат, салат жалбырагы, помидор, бадыра?, кинза, укроп. ',
    468,
    225,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Salattar/Midori_Sarada_salat/Medium.png?hash=021818dafad19245900c67a2a4c35905'
);

INSERT INTO products VALUES (
    52,
    6,
    'Морковь по-корейски',
    'МОРКОВНЫЙ САЛАТ ПО-КОРЕЙСКИ КОРЕЙ САБИЗ САЛАТЫ ',
    138,
    260,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v63/Salatyi/Morkov_po-koreiski/Medium.png?hash=7d79060236a1927fc4572cfe19f941e2'
);

INSERT INTO products VALUES (
    53,
    6,
    'Огурцы с говядиной по-китайски',
    'Огурцы, болгарский перец, лук, отварная говядина, чеснок, соевый соус, кинза. БАДЫРА? ЖАНА УЙ ЭТИНЕН ЖАСАЛГАН КЫТАЙ САЛАТЫ Бадыра?, болгар калемпири, пияз, кайнатылган уй эти, сарымсак, соя чыгы, кинза. Cucumber with beef in Chinese. Cucumbers, bell pepper, onions, boiled beef, garlic, soy sauce, cilantro.',
    468,
    320,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Salatyi/Ogurtsyi_s_goviadinoi_po-kitaiski/Medium.png?hash=c30c82ee9091d7b59a0ebff300a30f4f'
);

INSERT INTO products VALUES (
    54,
    6,
    'Оливье салат',
    'Отварной картофель, морковь и куриное яйцо, маринованные огурцы, зелёный горошек, докторская колбаса. Заправлен майонезом. ОЛИВЬЕ Сууга бышкан карт?шк?, сабиз жана тооктун жумурткасы, маринаддалган бадыра?, жашыл буурчак, доктор колбасасы. Майонез менен аралаштырылат. Olivier salad. Boiled potatoes, carrots and chicken eggs, pickled cucumbers and green peas, sausage «doktorskaya». With mayonnaise',
    318,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Salatyi/Olive_salat/Medium.png?hash=d696c09cc4ff1fc2fcf6b121f511ac90'
);

INSERT INTO products VALUES (
    55,
    7,
    'Ашлям-Фу',
    'Ашлям-фу с говядиной. Холодный острый овощной суп с крахмалом и лапшой. Подаётся с приправой Лаза. Холодный острый овощной суп с крахмалом и лапшой. Подаётся с приправой лаза. УЙ ЭТИ МЕНЕН АШЛЯМ-ФУ Крахмал, кесме жана жашылчалар кошулган ачуу муздак сорпо. Лаза татымалы менен берилет. Ashlyam Fu with beef. Cold spicy vegetable soup with starch and noodles. Served with seasoning Laza. ',
    238,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Supyi/Ashliam-Fu/Medium.png?hash=3dfdb0c55b75043851d6175451fe6e4a'
);

INSERT INTO products VALUES (
    56,
    7,
    'Борщ',
    'Подаётся с чесночными сухариками и сметаной. БОРЩ Сарымсактуу кургатылган нандар жана каймак менен берилет. Borsch. Served with garlic croutons and sour cream. ',
    418,
    430,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Supyi/Borshch/Medium.png?hash=5a4ade6e4833f57818f5149b77dcd2cf'
);

INSERT INTO products VALUES (
    57,
    7,
    'Китайский суп',
    'Говяжий бульон, говядина, древесные грибы, огурцы, стебель сельдерея, имбирь, перец чили, фунчоза. КЫТАЙ ШОРПОСУ Уйдун сорпосу, уй эти, дарак козу карындары, бадыра?, сельдерейдин с??г?г?, имбирь, чили калемпири, фунчоза. Japanese noodle soup Beef broth, beef, tree mushrooms, cucumbers, celery stalk, ginger, chili, funcheza',
    388,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v39/Supyi/Kitaiskii_sup/Medium.png?hash=83832b1dc158b52d3be21bda9eb30061'
);

INSERT INTO products VALUES (
    58,
    7,
    'Кукси',
    'Национальное блюдо корейской кухни. Бульон мури, яичная лапша, омлет, огурцы, помидоры, говядина, кунжут. Подаётся с приправой лаза. КУКСИ Корей ашканасынын улуттук тамагы. Мури сорпосу, жумуртка кошулган кесме, омлет, бадыра?, помидор, уй эти, к?нж?т. Лаза татымалы менен берилет. Kuksi. The national dish of Korean cuisine. Muri broth, egg noodles, scrambled eggs, cucumbers, tomatoes, beef, sesame. Served with seasoning Laza. ',
    388,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Supyi/Kuksi/Medium.png?hash=8317e270de05b3e9f73ed63bf8f90fa0'
);

INSERT INTO products VALUES (
    59,
    7,
    'Мампар',
    'Традиционный среднеазиатский суп из говядины с тестом. МАМПАР Камыр менен уй эти кошулган салттуу Орто Азия шорпосу. Mampar. Traditional Central Asian beef soup with dough.',
    368,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Supyi/Mampar/Medium.png?hash=f13b60a1f2a2bd3661e4fc919d9d9f44'
);

INSERT INTO products VALUES (
    60,
    7,
    'Окрошка',
    'Подаётся холодной, с горчицей и лимоном. КАЙМАКТА ЖАСАЛГАН ЭТ ОКРОШКАСЫ Муздак, горчица жана лимон менен берилет Meat Okroshka on sour cream. Boiled potatoes, fresh cucumbers, green onions, dill, eggs, meat, kefir. Served cold with mustard. ',
    348,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Supyi/Okroshka/Medium.png?hash=46eec9ff550a43b6d5e0bd04fd16ba73'
);

INSERT INTO products VALUES (
    61,
    7,
    'Острый Суп Том Ям',
    'Острый суп по-тайски. Куриное филе, помидоры, грибы вёшенки, креветки, шампиньоны, кинза, лимон. Подается с рисом. ТОМ-ЯМ Таиланд ачуу шорпосу. Тооктун сулп эти, помидор, вёшенки козу карыны, креветка, шампиньон, кинза, лимон. К?р?ч менен берилет. Tom Yum soup. Spicy Thai soup.Chicken broth, tomatoes, oyster mushrooms, shrimps, coconut milk, ginger root, Jalapeno hot pepper, cilantro, lime leaves. ',
    538,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Soup/Ostryii_Sup_Tom_Iam/Medium.png?hash=509c53783e70d1fd52432dd8cba5e040'
);

INSERT INTO products VALUES (
    62,
    7,
    'Острый суп Кимчи Тиге',
    'АЧУУ КИМЧИ ТИГЕ ШОРПОСУ Уй эти, кимчи, тофу, пияз, сарымсак, ачуу пастасы, соя пастасы, кинза. К?р?ч менен берилет. ОСТРЫЙ СУП КИМЧИ ТИГЕ Говядина, кимчи, тофу, лук, чеснок, острая паста, соевая паста, кинза. Подается с рисом. ',
    528,
    600,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v302/Soup/Ostryii_sup_Kimchi_Tige/Medium.png?hash=c64a8e15f5b6c764e3a22711f4fa3de8'
);

INSERT INTO products VALUES (
    63,
    7,
    'Пельмени с бульоном',
    'Подаются со сметаной и чесноком. ШОРПОЛУУ ПЕЛЬМЕНДЕР Каймак жана сарымсак менен берилет.',
    348,
    440,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Supyi/Pelmeni_s_bulonom/Medium.png?hash=d35083dbb268e686ef3c1c70467749d2'
);

INSERT INTO products VALUES (
    64,
    7,
    'Рамен классический',
    'Классический корейский рамен на остром бульоне с запеченной курицей «Су вид», водорослями Вакаме, капустой кимчи и яйцом Пашот. КЛАССИКАЛЫК РАМЕН Отко бышкан «Су вид» тоок эти, Вакаме балыры, кимчи капустасы жана Пашот жумурткасы менен ачуу сорподогу классикалык корей рамёну. ',
    468,
    650,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Soup/Ramen_klassicheski/Medium.png?hash=1be96a1b8b017c9f62177cfe08201040'
);

INSERT INTO products VALUES (
    65,
    8,
    'Босо Лагман',
    'Кусочки молодой говядины, пекинской капусты, и полугорького перца заправленный фирменной заправкой и специями. Подаётся с приправой лаза и китайским уксусом. БОСО ЛАГМАН Жаш уй эти, пекин капустасы жана жарым ачуу калемпирдин б?л?кт?р? менен татытылган. Лазататымалдары жана кытай уксусу менен берилет. ',
    518,
    370,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Yesyik_tamaktar/Boso_Lagman/Medium.png?hash=e9c7c9cc16b451f6b9c9e01d5c3e0b8b'
);

INSERT INTO products VALUES (
    66,
    8,
    'Вареники с картошкой по-украински',
    'Подаются со сметаной и жареным луком. КАРТ?ШК? МЕНЕН УКРАИНА ВАРЕНИГИ Каймак жана куурулган пияз менен берилет. Dumplings served with sour cream and fried onions.',
    268,
    230,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Goriachie_bliuda/Vareniki_s_kartoshkoi_po-ukrainski/Medium.png?hash=4119aed42f58e029ac9d5d00739452e9'
);

INSERT INTO products VALUES (
    67,
    8,
    'Ган-Фан',
    'Говядина, пекинская капуста, сладкий и полугорький перец, сельдерей, чеснок. Подается с рисом. ГАН-ФАН Уй эти, пекин капустасы, таттуу жана жарым ачуу калемпир, сельдерей, сарымсак. К?р?ч менен берилет. ',
    378,
    220,
    'Говядина, пекинская капуста, сладкий и полугорький перец, сельдерей, чеснок. Подается с рисом. ГАН-ФАН Уй эти, пекин капустасы, таттуу жана жарым ачуу калемпир, сельдерей, сарымсак. К?р?ч менен берилет. '
);

INSERT INTO products VALUES (
    68,
    8,
    'Голубцы',
    'Сочные голубцы, обернутые в нежные капустные листья и наполненные ароматной начинкой, подаются в томатном соусе. Подаются со сметаной. ГОЛУБЦЫЛАР Назик капуста жалбырактарына оролгон жана жыттуу толтурма салынган ширел?? голубцылар томат соусу менен берилет. Кам каймак менен берилет. ',
    388,
    330,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v219/Yesyik_tamaktar/Golubtsyi/Medium.png?hash=436d1de6b3fdbc0cd75645b188a60224'
);

INSERT INTO products VALUES (
    69,
    8,
    'Гречка с мясом',
    'Говядина, гречка. ЭТ МЕНЕН ГРЕЧКА Уй эти, гречка. Buckwheat with meat. Buckwheat, beef, onions, greens. ',
    388,
    260,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Yesyik_tamaktar/Grechka_s_miasom/Medium.png?hash=c876ae7d65e9263e34bef9bb61b08934'
);

INSERT INTO products VALUES (
    70,
    8,
    'Гречневая лапша с курицей',
    'Гречневая лапша, курица, кабачки, лук, болгарский перец, брокколи, кунжут, соус терияки. ТООК ЭТИ МЕНЕН ГРЕЧКА КЕСМЕСИ Гречка кесмеси, тоок эти, кабачоктор, пияз, болгар калемпири, брокколи, к?нж?т, терияки чыгы',
    508,
    350,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Goriachie_bliuda/Grechnevaia_lapsha_s_kuritsei/Medium.png?hash=7ad00961ed93de47b9613ea3e9503889'
);

INSERT INTO products VALUES (
    71,
    8,
    'Гёдза жареные с говядиной',
    'Гёдза жареные с говядиной Подаётся с чесночно-соевым соусом УЙ ЭТИ МЕНЕН КУУРУЛГАН ГЁДЗА Сарымсак-соя чыгы менен берилет. Fried beaf gyoza Served with garlic and soy sauce',
    348,
    350,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Goriachie_bliuda/Gdza_zharenyie_s_goviadinoi/Medium.png?hash=12d2c1d531f2da7caadaa4e44f16d1cc'
);

INSERT INTO products VALUES (
    72,
    8,
    'Гёдза с курицей на пару',
    'Подаются с фирменным острым соусом. ТООК ЭТИ МЕНЕН БУУГА БЫШКАН ГЁДЗА Фирмалык ачуу чык менен берилет. ',
    308,
    350,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Yesyik_tamaktar/Gdza_na_paru_s_kuritsei/Medium.png?hash=2f0b62996f5341eebe5e40165385bce4'
);

INSERT INTO products VALUES (
    73,
    8,
    'Дапанджи',
    'Горячее блюдо восточной кухни из пряной курицы с овощами и лапшой. Умеренной остроты. ДАПАНДЖИ Даамдуу тоок этинен бышырылган кесме жана жашылча аралаш орто ачуу ысык чыгыш тамагы. Roast chicken A fragrant hot dish with a rich taste. Fried chicken with bell peppers and fresh tomatoes. ',
    408,
    430,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Yesyik_tamaktar/Dapandzhi/Medium.png?hash=1e40badfda06e592f990f64a81335f7a'
);

INSERT INTO products VALUES (
    74,
    9,
    'Гречка отварная ',
    'Гречка отварная. Гречка, вода, соль, масло растительное. Buckwheat. Buckwheat, water, salt, vegetable oil.',
    98,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Garnirler/Grechka_otvarnaia/Medium.png?hash=1705e41d3afd3b8476cab705c0dbc06d'
);

INSERT INTO products VALUES (
    75,
    9,
    'Картофель Фри',
    'Картофель фри. Картофель, масло растительное, специи, майонез, кетчуп. КАРТ?ШК? ФРИ French fries. Potatoes, vegetable oil, spices, mayonnaise, ketchup',
    298,
    150,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Garniryi/Kartofel_Fri/Medium.png?hash=3f0511e5d7b9158d1486cff18571215e'
);

INSERT INTO products VALUES (
    76,
    9,
    'Картофель по-деревенски',
    'Картофель по-деревенски. Картофель, масло растительное, специи, майонез, кетчуп. ЭЛЕТ КАРТ?ШК?С? Fried potatoes Potatoes, vegetable oil, spices, mayonnaise, ketchup. ',
    258,
    120,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Garniryi/Kartofel_po-derevenski/Medium.png?hash=927d431ef26d16c2951b77155d9e3642'
);

INSERT INTO products VALUES (
    77,
    9,
    'Пюре картофельное',
    'КАРТ?ШК? пюреси КАРТОФЕЛЬНОЕ ПЮРЕ Mashed potatoes. ',
    98,
    150,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v337/Garniryi/Piure_kartofelnoe/Medium.png?hash=689a6a685e3e296f6ace33b3ab6bd134'
);

INSERT INTO products VALUES (
    78,
    9,
    'Рис отварной',
    'Рис отварной. Рис, вода, кунжут. Rice. Rice, water, sesame',
    98,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Garniryi/Ris_otvarnoi/Medium.png?hash=e35c4b6b9274df27fe6125a8947ccd59'
);

INSERT INTO products VALUES (
    79,
    9,
    'Тушеная капуста',
    'БЫШЫРЫЛГАН КАПУСТА ТУШЕНАЯ КАПУСТА ',
    98,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v332/Garnirler/Tushenaia_kapusta/Medium.png?hash=308dde50549d08e47df3060ae40566b6'
);

INSERT INTO products VALUES (
    80,
    9,
    'Тяхан с овощами',
    'Отварной рис, фирменный чесночный соус, болгарский перец, морковь, зелёный лук. ЖАШЫЛЧАЛАР МЕНЕН ТЯХАН Сууга бышкан к?р?ч, ?зг?ч?л?? сарымсак соусу, болгар калемпири, сабиз, к?к пияз. Tyahan with vegetables. Rice, green onions, onions, carrots, vegetable oil, Bulgarian pepper, spices, sauce. ',
    188,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Garniryi/Tiakhan_s_ovoshchami/Medium.png?hash=5b23c80a7cda490aa9371077574759d1'
);

INSERT INTO products VALUES (
    81,
    10,
    'Горчица 10.',
    '',
    58,
    10,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v21/Sousyi/Gorchitsa_50gr/Medium.png?hash=f1a7d01049e2faed2be2b643d85840d9'
);

INSERT INTO products VALUES (
    82,
    10,
    'Кетчуп ',
    '',
    58,
    30,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v21/Sousyi/Ketchup_50gr/Medium.png?hash=2f340a6630594353a1da0bba651a83c5'
);

INSERT INTO products VALUES (
    83,
    10,
    'Майонез',
    '',
    58,
    30,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v22/Sousyi/Maionez_50gr/Medium.png?hash=fe49d9ec61801587df7cdf0a1cc0ca2f'
);

INSERT INTO products VALUES (
    84,
    10,
    'Сметана',
    '',
    58,
    30,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v22/Sousyi/Smetana/Medium.png?hash=0b6970d3e2978e8834b1b937c6e0e56b'
);

INSERT INTO products VALUES (
    85,
    10,
    'Соус "Кесадилья"',
    '',
    68,
    30,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v21/Sousyi/Sous_Kesadilia/Medium.png?hash=e84518cf38c2cd9121e7791c74165e14'
);

INSERT INTO products VALUES (
    86,
    10,
    'Соус для мант',
    '',
    68,
    50,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Sousyi/Sous_dlia_mant/Medium.png?hash=097109125545db11ae87cc345bd7870d'
);

INSERT INTO products VALUES (
    87,
    11,
    'Балтимор ролл',
    'Жареный лосось, крабовые палочки, огурец, соус хот, тобико. БАЛТИМОР Куурулган лосось, краб таякчалары, бадыра?, хот чыгы, тобико.',
    438,
    175,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v49/Rollyi/Baltimor_roll/Medium.png?hash=9c9ca60ec45865718e04dca03acfcc54'
);

INSERT INTO products VALUES (
    88,
    11,
    'Гома ролл',
    'Куриная грудка, помидор, лук зеленый, соус кунжутный, кунжут. ГОМА РОЛЛ Тооктун т?ш эти, помидор, к?к пияз, к?нж?т чыгы,к?нж?т. Goma roll Chicken breast, tomato, green onion, sesame sauce, sesame seeds. ',
    338,
    185,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v40/Rollyi/Goma_roll/Medium.png?hash=6b085d22f5cce622602134cbad5b0016'
);

INSERT INTO products VALUES (
    89,
    11,
    'Горячий ролл Барбекю',
    'Колбаса полукопченая, помидор, огурец, перец, кетчуп барбекю. БАРБЕКЮ ЫСЫК РОЛЛУ Жарым-жартылай ышталган колбасалар, помидор, бадыра?, калемпир, барбекю чыгы Hot roll BBQ Semi-smoked sausage, tomato, cucumber, pepper, barbecue ketchup. ',
    378,
    220,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v40/Rollyi/Goriachii_roll_Barbekiu/Medium.png?hash=50be6436627c2d795d3388f08ca7a706'
);

INSERT INTO products VALUES (
    90,
    11,
    'Горячий ролл Тори терияки',
    'Куриная грудка, соус терияки, огурец, зелёный лук, омлет. ТОРИ ТЕРИЯКИ ЫСЫК РОЛЛУ Тооктун т?ш эти, терияки чыгы, бадыра?, к?к пияз, омлет. Hot roll tori teriyaki Chicken breast in teriyaki sauce, cucumber, spring onion, apanese omelet ',
    368,
    185,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Rollyi/Goriachii_roll_Tori_teriiaki/Medium.png?hash=d7f30df67f398ea264b53cb5526c7fd1'
);

INSERT INTO products VALUES (
    91,
    11,
    'Горячий ролл манго соус',
    ' ГОРЯЧИЙ РОЛЛ С МАНГОВЫМ СОУСОМ Нори, креветка темпура, соус чили, нити чили МАНГО СОУСУ МЕНЕН ЫСЫК РОЛЛ Нори, темпура креветкасы, чили соусу, чили жиби',
    578,
    230,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v303/Rolldor/Goriachii_roll_mango_sous/Medium.png?hash=c567fc2be90cd083fb1bea9b67d05732'
);

INSERT INTO products VALUES (
    92,
    11,
    'Запеченный Ролл Кавасаки',
    'Жареный лосось, крабовые палочки, огурец, омлет, соус xот. ОТКО БЫШКАН КАВАСАКИ РОЛЛУ Куурулган лосось, краб таякчалары, бадыра?, омлет, xот чыгы. ',
    388,
    185,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Rollyi/Zapechennyii_Roll_Kavasaki/Medium.png?hash=f376d66b58b3b240d77d1c95b642225e'
);

INSERT INTO products VALUES (
    93,
    11,
    'Запеченный Ролл Каджи Ёми',
    'Крабовые палочки, крем-чиз, кукуруза, зелёный лук, огурец, омлет, соус хот. ОТКО БЫШКАН КАДЖИ ЁМИ РОЛЛУ Краб таякчалары, крем-чиз, ж?г?р?, к?к пияз, бадыра?, омлет, хот чыгы. Baked Roll Kaji Yomi Crab sticks, cream cheese, corn, green onions, cucumber, omelette, Hot sauce ',
    348,
    210,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Rollyi/Zapechennyii_Roll_Kadzhi_mi/Medium.png?hash=f5a313d64ad3590486fc0358ab9200eb'
);

INSERT INTO products VALUES (
    99,
    11,
    'Запеченный Ролл Сакура',
    'Крем-чиз, крабовые палочки, кунжут, томаго, унаги соус, тобико. ОТКО БЫШКАН САКУРА РОЛЛУ Крем-чиз, краб таякчалары, к?нж?т, томаго, унаги чыгы, тобико. ',
    388,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Rolldor/Zapechennyii_Roll_Sakura/Medium.png?hash=f35b72d0bb947304ebfb02945a949c03'
);

INSERT INTO products VALUES (
    100,
    11,
    'Запеченный Ролл Хоте Кани',
    'Лосось, огурцы свежие, крабовые палочки, крем-чиз, соусы xот и унаги. ОТКО БЫШКАН ХОТЕ КАНИ РОЛЛУ Лосось, бадыра?, краб таякчалары, крем-чиз, xот жана унаги чыгы',
    508,
    185,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Rollyi/Zapechennyii_Roll_Khote_Kani/Medium.png?hash=da0ecdde6d7af16249a7776ff69ce7a3'
);

INSERT INTO products VALUES (
    101,
    11,
    'Изуми ролл',
    'Жареный лосось, огурец. ИЗУМИ РОЛЛ Куурулган лосось, бадыра?.',
    338,
    115,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Rollyi/Izumi_roll/Medium.png?hash=a09dd1acfcd3b2a610b8c6d9a6259094'
);

INSERT INTO products VALUES (
    102,
    12,
    'Сет "Атами"',
    'Роллы: Калифорния, Тсурай нори, Хот чиз, Огурец. АТАМИ Роллдор: Калифорния, Тсурай нори, Хот чиз, Бадыра? Set "Atami" Rolls: California, Tsurai, Hot cheese, cucumber ',
    1158,
    660,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v29/Setyi/Set_Atami/Medium.png?hash=a5f4cffc4bc750d627a3f74628ef53ab'
);

INSERT INTO products VALUES (
    103,
    12,
    'Сет "Икагеми"',
    'ИКАГЁМИ Роллы: Тори терияки, Кавасаки, Каджи ёми. ИКАГЁМИ Роллдор: Тори терияки, Кавасаки, Каджи ёми. Set "Ikagemi" Rolls: Tori teriyaki, Kawasaki, Kanji yomi ',
    1038,
    600,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Setyi/Set_Ikagemi/Medium.png?hash=264d16bc1c9c69e86d3c24b321036f19'
);

INSERT INTO products VALUES (
    104,
    12,
    'Сет "Империя"',
    'Роллы: Изуми ролл, Хотару, Балтимор, Филадельфия. Запечённый ролл Кавасаки. ИМПЕРИЯ Роллдор: Изуми ролл, Хотару, Балтимор, Филадельфия. Кавасаки какталган роллу. Set "Empire" Rolls: Izumi, Hotaru, Baltimore, Philadelphia Baked roll: Kawasaki " ',
    1858,
    1098,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v29/Setyi/Set_Imperiia/Medium.png?hash=58a6ba74aa02379134322e217b0803bb'
);

INSERT INTO products VALUES (
    105,
    12,
    'Сет "Мизуми"',
    'Роллы: Изуми ролл, Тсурай нори, Хоте Кани, Огурец. МИЗУМИ Роллдор: Изуми ролл, Тсурай нори, Хоте Кани, Бадыра?. Set "Mizumi" Rolls: Izumi roll, Tsurai, Hotte Kani, Cucumber',
    1168,
    590,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v29/Setyi/Set_Mizumi/Medium.png?hash=16dfdfb6066bf3d9f22af64d9688d771'
);

INSERT INTO products VALUES (
    106,
    12,
    'Сет "Огами"',
    'Роллы: Ясиро, Микура, Тояма, Аригато. ОГАМИ Роллдор: Ясиро, Микура, Тояма, Аригато. ',
    1408,
    740,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v156/Setyi/Set_Ogami/Medium.png?hash=615dce0ccc4caea1f436a2778dfe15d8'
);

INSERT INTO products VALUES (
    107,
    12,
    'Сет "Окинава"',
    'Роллы: Римский, Комбу, Калифорния, Гома. ОКИНАВА Роллдор: Римский, Комбу, Калифорния, Гома. Okinawa New Rolls: Roman, Kombu, California, Goma. ',
    1168,
    700,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v40/Setyi/Set_Okinava/Medium.png?hash=3fd189625c77eff0fe47ec0b8bd00365'
);

INSERT INTO products VALUES (
    108,
    12,
    'Сет "Оригами"',
    'Роллы: Цезарь, Филадельфия, Касуми. ОРИГАМИ Роллдор: Цезарь, Филадельфия, Касуми Set "Origami" Rolls: Caesar, Philadelphia premium, Kasumi. ',
    1408,
    555,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v29/Setyi/Set_Origami/Medium.png?hash=fadb0355e5a8d8f185c41705c1140e02'
);

INSERT INTO products VALUES (
    109,
    12,
    'Сет "Фудзи Яма"',
    'Роллы: Хоте кани, Хот чиз, Юмико, Яки урамаки. ФУДЗИ ЯМА Роллдор: Хоте кани, Хот чиз, Юмико, Яки урамаки. Set "Fuji Yama" Rolls: Hotte Kani, Hot Cheese, Yumiko, Yaki Nori. ',
    1748,
    780,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v29/Setyi/Set_Fudzi_Iama/Medium.png?hash=fbff75719e5e35aa435f28124fe2678a'
);

INSERT INTO products VALUES (
    110,
    13,
    'Мохито Классический ГРАФИН',
    'Мята, лайм, сахарный сироп, вода газированная, Sprite Алкоголсуз Мохито Жалбыз, лайм, шекер ма?ызы, газдалган суу, Sprite ',
    398,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v343/Bezalkogolnyie_kokteili/Mokhito_Klassicheskii_GRAFIN/Medium.png?hash=b6b986768e7c54ea4a35367564c9ba20'
);

INSERT INTO products VALUES (
    111,
    13,
    'Мохито Классический безалкогольный',
    'Мята, лайм, сахарный сироп, вода газированная, Sprite Алкоголсуз Мохито Жалбыз, лайм, шекер ма?ызы, газдалган суу, Sprite ',
    258,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v318/Non-alcoholic_cocktails/Mokhito_Klassicheskii_bezalkogolnyii/Medium.png?hash=ff7f07adf76a9574b7d54070e8bdbbe1'
);

INSERT INTO products VALUES (
    112,
    13,
    'OREO',
    'Мороженое сливочное, молоко, печенье Oreo, шоколадный сироп, взбитые сливки OREO Каймактуу балмуздак, с?т, Oreo печеньеси, шоколад ма?ызы, чалып к?б?рт?лг?н камкаймак',
    298,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v318/Non-alcoholic_cocktails/OREO/Medium.png?hash=beea9b1c90fabdf778a1ace45898bd6d'
);

INSERT INTO products VALUES (
    113,
    13,
    'Айс Американо с лимоном',
    '',
    288,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v368/Alkogolsuz_suusunduktar/Ais_Amerikano_s_limonom/Medium.png?hash=012bfdee65dd4d5081058ebf918a6f90'
);

INSERT INTO products VALUES (
    114,
    14,
    'Американо кофе',
    'Варенный черный кофе',
    198,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Amerikano_kofe_Hausbrandt/Medium.png?hash=89cf354949f1510b10bbbf69dc8fb3bf'
);

INSERT INTO products VALUES (
    115,
    14,
    'Двойной Американо кофе',
    'Варенный черный кофе ',
    228,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Dvoinoi_Amerikano_kofe_Hausbrandt/Medium.png?hash=e7ccb8196a897589cc8484abc5ff7327'
);

INSERT INTO products VALUES (
    116,
    14,
    'Двойной Капучино кофе ',
    'Варенный кофе с добавлением молока и корицы ',
    278,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Dvoinoi_Kapuchino_kofe_Hausbrandt/Medium.png?hash=b79454651f3532b475096cf73d424938'
);

INSERT INTO products VALUES (
    117,
    14,
    'Капучино кофе',
    'Варенный кофе с добавлением молока и корицы',
    218,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Kapuchino_kofe_Hausbrandt/Medium.png?hash=f2fd872ca846f34c8b2b675496016207'
);

INSERT INTO products VALUES (
    118,
    14,
    'Латте кофе',
    'Молочный кофе, подается в слоистом виде.',
    228,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Latte_kofe_Hausbrandt/Medium.png?hash=fcda076338dbb983756ea993d717a2d9'
);

INSERT INTO products VALUES (
    119,
    14,
    'Латте на растительном молоке',
    'Латте на растительном молоке *овсяное или кокосовое на выбор ?с?мд?к с?тт?? Латте *тандоо?узга жараша сулу же кокос с?тт?? кофе',
    288,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Latte_na_rastitelnom_moloke/Medium.png?hash=fcda076338dbb983756ea993d717a2d9'
);

INSERT INTO products VALUES (
    120,
    14,
    'Матча-Латте на кокосовом молоке',
    'Матча чай, молоко кокосовое, сироп кокос Матча чай, кокос с?т?, кокос ма?ызы ',
    318,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v266/Kainatyilgan_suusunduktar/Matcha-Latte_na_kokosovom_moloke/Medium.png?hash=6d4220834e14a4997ca7b917c12e617a'
);

INSERT INTO products VALUES (
    121,
    15,
    'Варенье',
    'Варенье ягодное. Berry jam',
    98,
    50,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Desertyi/Varene_50gr/Medium.png?hash=5c1cc2ce0ba1a7bf23f18c7cb54ec67f'
);

INSERT INTO products VALUES (
    122,
    15,
    'Дубайский чизкейк',
    'ДУБАЙЛЫК ЧИЗКЕЙК ДУБАЙСКИЙ ЧИЗКЕЙК ',
    488,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v333/Desserts/Dubaiskii_chizkeik/Medium.png?hash=85fa02d059dd60c0b80774d8fce37744'
);

INSERT INTO products VALUES (
    123,
    15,
    'Круассан классический',
    'КРУАССАН КЛАССИЧЕСКИЙ КЛАССИКАЛЫК КРУАССАН',
    118,
    40,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Desertyi/Kruassan_klassicheskii_1sht/Medium.png?hash=a715f17fa328fb95b15fb9f45b5ea933'
);

INSERT INTO products VALUES (
    124,
    15,
    'Мед',
    'Мёд. Honey.',
    118,
    50,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Desertyi/Med_50gr/Medium.png?hash=7cc46b93cdc647adea1f53b40473358c'
);

INSERT INTO products VALUES (
    125,
    15,
    'Медовик',
    'Подается с грецкими орехами. Жа?гак менен берилет',
    388,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v60/Desertyi/Medovik/Medium.png?hash=ec5e543506c6e6e971dd7455b7737ff6'
);

INSERT INTO products VALUES (
    126,
    15,
    'Мороженое Ванильное (1 шарик)',
    'Фирменное мороженное (3 шарика). Brand Ice Cream (3 balls). ',
    68,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Desertyi/Morozhenoe_Vanilnoe_1_sharik/Medium.png?hash=14b0fff5cfd1e3de6f1a472d05362b6c'
);

INSERT INTO products VALUES (
    127,
    15,
    'Мороженое Ванильное (3 шарика)',
    'Фирменное мороженое (3 шарика). Brand Ice Cream (3 balls). ',
    178,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Desertyi/Morozhenoe_Vanilnoe_3_sharika/Medium.png?hash=9b8a5bf5450ea4f1409a49d4dba9110c'
);

INSERT INTO products VALUES (
    128,
    15,
    'Мороженое Ванильное со стружкой(1шарик)',
    'Фирменное мороженное (3 шарика). Brand Ice Cream (3 balls). ',
    68,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v21/Desertyi/Morozhenoe_Vanilnoe_so_struzhkoi_1sharik/Medium.png?hash=d398e0eaec4591ffb4adc33239eae97e'
);

INSERT INTO products VALUES (
    129,
    16,
    'Боорсок',
    'Боорсок. Boorsok (kyrgyz traditional fried bread pieces). ',
    198,
    280,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Khleb/Boorsok/Medium.png?hash=b91c165b3dfd7dc91656b8b605addc07'
);

INSERT INTO products VALUES (
    130,
    16,
    'Бородинский',
    'Бородинский хлеб. Один из самых распространённых сортов ржаного хлеба. В состав, помимо ржаной и пшеничной муки, входят солод, сахар и кориандр. Borodino bread. One of the most common varieties of rye bread. In addition to of rye and wheat flour it includes malt, sugar and cilantro.',
    48,
    2,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v277/Nan/Borodinskii/Medium.png?hash=2b52dedd56ee80da553a40415b92a8e5'
);

INSERT INTO products VALUES (
    131,
    16,
    'Лепешка',
    'Лепёшка. ТОКОЧ Lepyoshka (flat kyrgyz bread). ',
    118,
    300,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Khleb/Lepeshka/Medium.png?hash=057762ed0ac84bf6df6a0a330569a5b5'
);

INSERT INTO products VALUES (
    132,
    16,
    'Пампушки 1шт',
    '',
    48,
    200,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v40/Khleb/Pampushki_1sht/Medium.png?hash=d8da6d75252d1fc7544b3403c7e291fd'
);

INSERT INTO products VALUES (
    133,
    16,
    'Сырная лепешка',
    'с кинзой. Подаётся с фирменным соусом кесадилья и сметаной. СЫР ТОКОЧУ *кинза менен. Кесадилья фирмалык чыгы жана каймак менен берилет. Cheese bread. Served with Caesaldilla sauce and sour cream',
    448,
    430,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Khleb/Syirnaia_lepeshka/Medium.png?hash=e30dd23c51c654c541eb17123f26ea84'
);

INSERT INTO products VALUES (
    134,
    16,
    'Хлебное ассорти',
    'Корншпиц, зерновой, бородинкский хлеб АССОРТИ НАН Корншпиц, дандуу нан, бородино наны. ',
    198,
    8,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v250/bread/Khlebnoe_assorti/Medium.png?hash=e4a6dcc763166e6e9994d452206a7bd9'
);

INSERT INTO products VALUES (
    135,
    16,
    'Чесночные гренки',
    'Чесночные гренки. Подаются с сырным соусом. САРЫМСАКТУУ КУУРУЛГАН НАН Сыр-сарымсак чыгы менен берилет Garlic croutons. Served with cheese sauce. ',
    238,
    120,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Khleb/Chesnochnyie_grenki/Medium.png?hash=fb20aeefac042ea1592ed211ad7823e1'
);

INSERT INTO products VALUES (
    136,
    16,
    'Чесночный',
    'Чесночный хлеб. САРЫМСАКТУУ НАН Garlic bread',
    118,
    4,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v1/Khleb/Chesnochnyii_khleb/Medium.png?hash=c669d1858168ffd385cfd9a4d3e317a9'
);

INSERT INTO products VALUES (
    137,
    16,
    'Чиабатта',
    'Итальянский хлеб с хрустящей корочкой и воздушной мякотью. ЧИАБАТТА Кытырак кыртыштуу жана ичи жумшак италиян наны. Ciabatta  Italian bread with a crispy crust and airy pulp.',
    118,
    1,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Khleb/Chiabatta/Medium.png?hash=3744acd4d2c9837b4c56dfe1eb99a91a'
);

INSERT INTO products VALUES (
    138,
    17,
    'RICHMOND Bronze',
    '',
    388,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v163/Sigaretyi/RICHMOND_Bronze/Medium.png?hash=a65deb0e4b338f1480a7fe7039e14f69'
);

INSERT INTO products VALUES (
    139,
    17,
    'RICHMOND Red',
    '',
    358,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v16/Sigaretyi/RICHMOND_Red/Medium.png?hash=aeab5fda5449b2b5108034588e4df211'
);

INSERT INTO products VALUES (
    140,
    17,
    'Винстон Blue',
    '',
    358,
    100,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v16/Sigaretyi/Vinston_Blue/Medium.png?hash=aeab5fda5449b2b5108034588e4df211'
);

INSERT INTO products VALUES (
    141,
    18,
    'Fuse tea',
    '',
    178,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Kholodnyie_napitki/Fuse_tea_0_5l/Medium.png?hash=9e6ba101239e8adc2f1e10e1c3a40602'
);

INSERT INTO products VALUES (
    142,
    18,
    'Gracio сок',
    '',
    368,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v348/Kholodnyie_napitki/Gracio_sok_1l/Medium.png?hash=c4789f992b82e5cf6d269b3dcf216067'
);

INSERT INTO products VALUES (
    143,
    18,
    'J7 сок',
    '',
    318,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Kholodnyie_napitki/J7_sok_1l/Medium.png?hash=5614367a21b2dc1a799b1babff5d2733'
);

INSERT INTO products VALUES (
    144,
    18,
    'Piko',
    '',
    248,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Kholodnyie_napitki/Piko_sok_1l/Medium.png?hash=64bec074be3c7c5638b0fc634b134a16'
);

INSERT INTO products VALUES (
    145,
    18,
    'Red Bull',
    '',
    348,
    250,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Kholodnyie_napitki/Red_Bull/Medium.png?hash=72e5c677fae77406d6769f91511e90c0'
);

INSERT INTO products VALUES (
    146,
    18,
    'Shweppes',
    '',
    228,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v243/Muzdak_suusunduktar/Shweppes_tonik_0_45l/Medium.png?hash=1c9a9edf8b839ad4252374c28aa15d7f'
);

INSERT INTO products VALUES (
    147,
    18,
    'Borjomi',
    '',
    198,
    330,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v348/Kholodnyie_napitki/Borjomi_0_33l/Medium.png?hash=f0b733527b806f79fa95e63176703f43'
);

INSERT INTO products VALUES (
    148,
    18,
    'Nitro',
    'Первый отечественный энергетический напиток «Nitro». Молодежный бренд. Напиток для мобильных, ярких, спортивных, креативных и дерзких людей. Для изготовления энергетического напитка «Nitro» используются только качественные ингредиенты из Германии и артезианская вода.',
    198,
    450,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v10/Kholodnyie_napitki/Nitro/Medium.png?hash=7e26343f1ed0c62b40d4bae615db7db0'
);

INSERT INTO products VALUES (
    149,
    19,
    'Carlsberg',
    'ПРЕДУПРЕЖДАЕМ О ВРЕДЕ ЧРЕЗМЕРНОГО ПОТРЕБЛЕНИЯ АЛКОГОЛЬНОЙ ПРОДУКЦИИ',
    288,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v164/Pivo/Carlsberg_but/Medium.png?hash=3fbe924babffb3d49e00e4f69aa33850'
);

INSERT INTO products VALUES (
    150,
    19,
    'Corona Extra Zero',
    'Пиво производство: Мексика • ПРЕДУПРЕЖДАЕМ О ВРЕДЕ ЧРЕЗМЕРНОГО ПОТРЕБЛЕНИЯ АЛКОГОЛЬНОЙ ПРОДУКЦИИ •',
    428,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v348/Pivo/Corona_Extra_Zero_but/Medium.png?hash=b43fb9a2074408b304d5b8e5aee0a780'
);

INSERT INTO products VALUES (
    151,
    20,
    'Emilio Bianco Dry',
    '• ПРЕДУПРЕЖДАЕМ О ВРЕДЕ ЧРЕЗМЕРНОГО ПОТРЕБЛЕНИЯ АЛКОГОЛЬНОЙ ПРОДУКЦИИ •',
    2618,
    1000,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v243/Vino_Shampanskoe/Emilio_Bianco_Dry_beloe_sukhoe_but/Medium.png?hash=75c854bb4c696eb3899299a9f4b8747d'
);

INSERT INTO products VALUES (
    152,
    21,
    'Арарат 5 звёзд',
    '• ПРЕДУПРЕЖДАЕМ О ВРЕДЕ ЧРЕЗМЕРНОГО ПОТРЕБЛЕНИЯ АЛКОГОЛЬНОЙ ПРОДУКЦИИ •',
    3478,
    500,
    'https://staticcontent.mypizza.kg/Dishes/Imperiia_Pitstsyi/v237/Elitnyie_spirtnyie_napitki/Ararat_5_zvzd_but_0_5l/Medium.png?hash=585ae803ee0bf0510fe263626dfb07de'
);

DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id          BIGSERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    created_at  TIMESTAMP DEFAULT NOW()
);

CREATE TABLE favorites (
    id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    UNIQUE(user_id, product_id)
);


CREATE TABLE cart_items (
    id SERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE,
    product_id INTEGER REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1,
    UNIQUE(user_id, product_id)
);