-- SurrealDB migration generated from database.db
-- Run with: surreal sql --conn http://localhost:8000 --user root --pass root --ns myns --db mydb < migrate.surql

-- ==================== SCHEMA ====================

DEFINE TABLE location_map SCHEMAFULL;
DEFINE FIELD location_id   ON location_map TYPE int;
DEFINE FIELD location_name ON location_map TYPE string;

DEFINE TABLE people SCHEMAFULL;
DEFINE FIELD uid      ON people TYPE string;
DEFINE FIELD greeting ON people TYPE string;
DEFINE FIELD name     ON people TYPE string;
DEFINE FIELD line1    ON people TYPE string;
DEFINE FIELD line2    ON people TYPE option<string>;
DEFINE FIELD line3    ON people TYPE option<string>;
DEFINE FIELD desc     ON people TYPE option<string>;
DEFINE FIELD depart_from ON people TYPE option<record<location_map>>;

-- ==================== location_map ====================

CREATE location_map:1 SET
    location_id   = 1,
    location_name = 'FPT Tower';
CREATE location_map:2 SET
    location_id   = 2,
    location_name = 'Handico Tower';
CREATE location_map:3 SET
    location_id   = 3,
    location_name = 'Lotte mall West Lake';
CREATE location_map:4 SET
    location_id   = 4,
    location_name = 'I use my own vehicle';
CREATE location_map:5 SET
    location_id   = 5,
    location_name = 'I\'ll pass';

-- ==================== people ====================

CREATE people:32985cff SET
    uid          = '32985cff',
    greeting     = 'Oi',
    name         = 'Hoàng',
    line1        = 'Guess what, I\'m getting married after all...',
    line2        = 'Would be awesome to have you there!',
    line3        = NONE,
    desc         = 'Hoàng shitpost',
    depart_from  = location_map:1;
CREATE people:769a53cc SET
    uid          = '769a53cc',
    greeting     = 'Yo, my G',
    name         = 'Bách',
    line1        = 'It\'s kinda weird to say, but well... I\'m getting married!',
    line2        = 'Would love you to be there.',
    line3        = NONE,
    desc         = 'Bách shitpost',
    depart_from  = location_map:1;
CREATE people:48fa6f6c SET
    uid          = '48fa6f6c',
    greeting     = 'Hi',
    name         = 'Hoàng',
    line1        = 'It\'s me! I\'m getting married.',
    line2        = 'Hope you\'ll be there to celebrate with us.',
    line3        = NONE,
    desc         = 'Hoàng LGE',
    depart_from  = location_map:5;
CREATE people:b814c110 SET
    uid          = 'b814c110',
    greeting     = 'Sup',
    name         = 'Tuyết',
    line1        = 'Your Linux brother is getting married xD',
    line2        = 'Let\'s come and celebrate!',
    line3        = NONE,
    desc         = 'Tuyết Rikkei',
    depart_from  = location_map:2;
CREATE people:139c3750 SET
    uid          = '139c3750',
    greeting     = 'Ayyy',
    name         = 'Tân',
    line1        = 'Guess who’s escaping the single life?',
    line2        = 'This lonely era is officially over—I’m getting married!',
    line3        = 'Come celebrate with us, because it wouldn’t be the same without you.',
    desc         = 'Tân Rikkei',
    depart_from  = location_map:2;
CREATE people:3d4ddcf8 SET
    uid          = '3d4ddcf8',
    greeting     = 'Yooo',
    name         = 'Ngọc',
    line1        = 'Time really flies, doesn’t it?',
    line2        = 'I\'m about to turn the page to a new chapter—I\'m getting married!',
    line3        = 'It would mean so much to have you there with us.',
    desc         = 'Ngọc rikkei',
    depart_from  = location_map:2;
CREATE people:aea13217 SET
    uid          = 'aea13217',
    greeting     = 'Hehe hello',
    name         = 'anh Hưng',
    line1        = 'Guess what? Your little brother is beating you to the altar!',
    line2        = 'I’d be so happy to have you there with us on that special day.',
    line3        = 'Hope you can make it!',
    desc         = 'A Hưng Rikkei',
    depart_from  = location_map:2;
CREATE people:501c7893 SET
    uid          = '501c7893',
    greeting     = 'Dear',
    name         = 'chị Thảo',
    line1        = 'Time really flies, doesn’t it?',
    line2        = 'I’ll always be grateful for the time I got to be part of your team—and now, I’m getting married!',
    line3        = 'I’d be so happy if you could join me on that day.',
    desc         = 'Chị Thảo Dương',
    depart_from  = location_map:1;
CREATE people:4fd0cce3 SET
    uid          = '4fd0cce3',
    greeting     = 'Hola',
    name         = 'chị Thảo',
    line1        = 'How have you been?',
    line2        = 'The time I worked with you was such a joy—and now, I\'m getting married!',
    line3        = 'Really hope you can join us on that day.',
    desc         = 'Chị Thảo bé',
    depart_from  = location_map:2;
CREATE people:43bb7db3 SET
    uid          = '43bb7db3',
    greeting     = 'Ayo',
    name         = 'chị Thùy',
    line1        = 'Your little "Hello mọi người" is getting married—surprise!',
    line2        = 'Would love for you to come celebrate with me.',
    line3        = 'Hope you can make it!',
    desc         = 'C Thùy Rikkei',
    depart_from  = location_map:2;
CREATE people:1677f46e SET
    uid          = '1677f46e',
    greeting     = 'Hiiii',
    name         = 'anh Độ',
    line1        = 'Thank you for always being so kind to everyone—even when I couldn\'t help much :D',
    line2        = 'The time I worked with you all was truly special.',
    line3        = 'I\'m getting married—it would mean so much to me if you could come celebrate with me.',
    desc         = 'A Độ Rikkei',
    depart_from  = location_map:2;
CREATE people:1d73060d SET
    uid          = '1d73060d',
    greeting     = 'Ayo',
    name         = 'Hải',
    line1        = 'Long time no see—still having fun at CMC? ',
    line2        = 'I\'m getting married!',
    line3        = 'Your presence would honestly make my day.',
    desc         = 'Hải thạc sĩ',
    depart_from  = location_map:1;
CREATE people:e96d4a54 SET
    uid          = 'e96d4a54',
    greeting     = 'Wazzap',
    name         = 'Sơn',
    line1        = 'It\'s been months since we last hung out—yeah, my office is pretty far now',
    line2        = 'Anyway, I\'m getting married!',
    line3        = 'Hope you can make it, would love to catch up.',
    desc         = 'Sơn bún bò',
    depart_from  = location_map:1;
CREATE people:4039d9d4 SET
    uid          = '4039d9d4',
    greeting     = 'Yo',
    name         = 'Quyết',
    line1        = 'Been a minute since we last gathered—work\'s got me way out there now ',
    line2        = 'Anyway, I\'m getting married!',
    line3        = 'Hope you can make it, miss those food trips with the crew.',
    desc         = 'Quyết PM',
    depart_from  = location_map:1;
CREATE people:7e9aa418 SET
    uid          = '7e9aa418',
    greeting     = 'Hey',
    name         = 'Thành',
    line1        = 'It\'s been a while since we last caught up—work keeps me pretty far now ',
    line2        = 'Just wanted to let you know I\'m getting married!',
    line3        = 'Would be great if you could make it.',
    desc         = 'Thành Linux',
    depart_from  = location_map:5;
CREATE people:828c7355 SET
    uid          = '828c7355',
    greeting     = 'Sup',
    name         = 'my G',
    line1        = 'Guess it\'s my turn now—I\'m getting married! ',
    line2        = 'Would love for you and your wife to come celebrate with us.',
    line3        = 'Can\'t wait to have both of you there!',
    desc         = 'Phan & Hảo',
    depart_from  = location_map:4;
CREATE people:d703932b SET
    uid          = 'd703932b',
    greeting     = 'Hey',
    name         = 'Ly',
    line1        = 'Life moves fast, huh? I\'m getting married!',
    line2        = 'Would really love for you to come celebrate with me.',
    line3        = 'Wishing you nothing but the best always.',
    desc         = 'Lee',
    depart_from  = location_map:5;
CREATE people:1a19a1db SET
    uid          = '1a19a1db',
    greeting     = 'Yo',
    name         = 'Đức',
    line1        = 'Long time no see, but glad we still keep in touch ',
    line2        = 'Just wanted to let you know—I\'m getting married!',
    line3        = 'Would love for you and your wife to come celebrate with us.',
    desc         = 'Đức Milo',
    depart_from  = location_map:4;
CREATE people:1a25a31b SET
    uid          = '1a25a31b',
    greeting     = 'Hey',
    name         = 'Nhiệm',
    line1        = 'Heard you\'re back in town—about time!',
    line2        = 'I\'m getting married, so no pressure… but now it\'s your turn to find a GF and bring her along!',
    line3        = 'Hope you can both make it!',
    desc         = 'Phan tôm',
    depart_from  = location_map:4;
CREATE people:98a4576b SET
    uid          = '98a4576b',
    greeting     = 'Mr.',
    name         = 'Sơn',
    line1        = 'Small world—so glad we reconnected and now we\'re coworkers too',
    line2        = 'I\'m getting married!',
    line3        = 'Bring your girlfriend along, would love to have you both there!',
    desc         = 'Sơn LGE',
    depart_from  = location_map:1;
CREATE people:25c6dad3 SET
    uid          = '25c6dad3',
    greeting     = 'Yo',
    name         = 'Đăng',
    line1        = 'From ex-coworkers to starting at the new place around the same time—what a journey ',
    line2        = 'Now I\'m getting married!',
    line3        = 'Hope you can come celebrate with me!',
    desc         = 'Đăng CMC',
    depart_from  = location_map:1;
CREATE people:8c0ef8c5 SET
    uid          = '8c0ef8c5',
    greeting     = 'Yo',
    name         = 'Hằng',
    line1        = 'Time flies, huh? Remember when you used to gossip to my parents and get me beaten every single day?',
    line2        = 'Well, karma\'s here — now I\'m getting married. Wish you could come, but I know England\'s a bit far.',
    line3        = 'Just wanted to share the happiness with you. We\'ll party when you\'re home!',
    desc         = 'Hằng toong',
    depart_from  = location_map:5;
CREATE people:6ee1ca2d SET
    uid          = '6ee1ca2d',
    greeting     = 'Helu',
    name         = 'Tuân',
    line1        = 'Time flies so fast, doesn\'t it?',
    line2        = 'I know Korea is... well... far as f*ck, but just wanted to share the news with you.',
    line3        = 'We\'ll celebrate when you\'re back!',
    desc         = 'Tuân adknt',
    depart_from  = location_map:5;
CREATE people:8a371377 SET
    uid          = '8a371377',
    greeting     = 'Yo',
    name         = 'bro Quốc Anh',
    line1        = 'Well, I guess my carefree life is over — no more staying up late, no more gaming.',
    line2        = 'I\'m stepping into a new chapter of my life.',
    line3        = 'Hope you can come and join me on my big day.',
    desc         = 'Chanh FPT',
    depart_from  = location_map:5;
CREATE people:e7ff83f8 SET
    uid          = 'e7ff83f8',
    greeting     = 'Yo',
    name         = 'Hiển',
    line1        = 'Long time no see.',
    line2        = 'I know you\'re far away right now, so I don\'t expect you to make it — but I just wanted to let you know I\'m getting married.',
    line3        = 'Miss you, man. We\'ll hang out when you\'re back.',
    desc         = 'Hiển CHY',
    depart_from  = location_map:5;
CREATE people:6b82e1c5 SET
    uid          = '6b82e1c5',
    greeting     = 'Yo',
    name         = 'Huy',
    line1        = 'Long time no see. How\'s life?',
    line2        = 'I\'m getting married and would love for you to be there.',
    line3        = 'Hope you can make it!',
    desc         = 'Huy keyboard',
    depart_from  = location_map:1;
CREATE people:cfcd65b7 SET
    uid          = 'cfcd65b7',
    greeting     = 'Helu',
    name         = 'Lan Anh',
    line1        = 'Still hunting for a new job, huh?',
    line2        = 'Well, I\'ve got some news — I\'m getting married!',
    line3        = 'Would love for you to be there with us. Hope you can make it!',
    desc         = 'ELA',
    depart_from  = location_map:2;

-- ==================== DONE ====================
