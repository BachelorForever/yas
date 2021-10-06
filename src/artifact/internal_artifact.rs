use regex::Regex;
use std::hash::{Hash, Hasher};
use edit_distance;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ArtifactStatName {
    HealingBonus,
    CriticalDamage,
    Critical,
    Atk,
    AtkPercentage,
    ElementalMastery,
    Recharge,
    HpPercentage,
    Hp,
    DefPercentage,
    Def,
    ElectroBonus,
    PyroBonus,
    HydroBonus,
    CryoBonus,
    AnemoBonus,
    GeoBonus,
    PhysicalBonus,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ArtifactSlot {
    Flower,
    Feather,
    Sand,
    Goblet,
    Head,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum ArtifactSetName {
    ArchaicPetra,
    HeartOfDepth,
    BlizzardStrayer,
    RetracingBolide,
    NoblesseOblige,
    GladiatorFinale,
    MaidenBeloved,
    ViridescentVenerer,
    LavaWalker,
    CrimsonWitch,
    ThunderSmoother,
    ThunderingFury,
    BloodstainedChivalry,
    WandererTroupe,
    Scholar,
    Gambler,
    TinyMiracle,
    MartialArtist,
    BraveHeart,
    ResolutionOfSojourner,
    DefenderWill,
    Berserker,
    Instructor,
    Exile,
    Adventurer,
    LuckyDog,
    TravelingDoctor,
    PrayersForWisdom,
    PrayersToSpringtime,
    PrayersForIllumination,
    PrayersForDestiny,
    PaleFlame,
    TenacityOfTheMillelith,
    EmblemOfSeveredFate,
    ShimenawaReminiscence,
}

#[derive(Debug, Clone)]
pub struct ArtifactStat {
    pub name: ArtifactStatName,
    pub value: f64,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct InternalArtifact {
    pub set_name: ArtifactSetName,
    pub slot: ArtifactSlot,
    pub star: u32,
    pub level: u32,
    pub main_stat: ArtifactStat,
    pub sub_stat_1: Option<ArtifactStat>,
    pub sub_stat_2: Option<ArtifactStat>,
    pub sub_stat_3: Option<ArtifactStat>,
    pub sub_stat_4: Option<ArtifactStat>,
    pub equip: Option<String>,
}

impl Hash for ArtifactStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let v = (self.value * 1000.0) as i32;
        v.hash(state);
    }
}

impl PartialEq for ArtifactStat {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        let v1 = (self.value * 1000.0) as i32;
        let v2 = (other.value * 1000.0) as i32;

        v1 == v2
    }
}

impl Eq for ArtifactStat {}

impl ArtifactStatName {
    pub fn from_zh_cn(name: &str, is_percentage: bool) -> Option<ArtifactStatName> {
        match name {
            "治疗加成" => Some(ArtifactStatName::HealingBonus),
            "暴击伤害" => Some(ArtifactStatName::CriticalDamage),
            "暴击率" => Some(ArtifactStatName::Critical),
            "攻击力" => if is_percentage { Some(ArtifactStatName::AtkPercentage) } else { Some(ArtifactStatName::Atk) },
            "元素精通" => Some(ArtifactStatName::ElementalMastery),
            "元素充能效率" => Some(ArtifactStatName::Recharge),
            "生命值" => if is_percentage { Some(ArtifactStatName::HpPercentage) } else { Some(ArtifactStatName::Hp) },
            "防御力" => if is_percentage { Some(ArtifactStatName::DefPercentage) } else { Some(ArtifactStatName::Def) },
            "雷元素伤害加成" => Some(ArtifactStatName::ElectroBonus),
            "火元素伤害加成" => Some(ArtifactStatName::PyroBonus),
            "水元素伤害加成" => Some(ArtifactStatName::HydroBonus),
            "冰元素伤害加成" => Some(ArtifactStatName::CryoBonus),
            "风元素伤害加成" => Some(ArtifactStatName::AnemoBonus),
            "岩元素伤害加成" => Some(ArtifactStatName::GeoBonus),
            "物理伤害加成" => Some(ArtifactStatName::PhysicalBonus),
            _ => None,
        }
    }
}

impl ArtifactStat {
    // e.g "生命值+4,123", "暴击率+10%"
    pub fn from_zh_cn_raw(s: &str) -> Option<ArtifactStat> {
        let temp: Vec<&str> = s.split("+").collect();
        if temp.len() != 2 {
            return None;
        }

        let is_percentage = temp[1].contains("%");
        let stat_name = match ArtifactStatName::from_zh_cn(temp[0], is_percentage) {
            Some(v) => v,
            None => return None,
        };

        let re = Regex::new("[%,]").unwrap();
        let mut value = re.replace_all(temp[1], "").parse::<f64>().unwrap();
        if is_percentage {
            value /= 100.0;
        }

        Some(ArtifactStat {
            name: stat_name,
            value,
        })
    }
}

pub fn get_real_artifact_name_chs(raw: &str) -> Option<String> {
    let all_artifact_chs = [
        "磐陀裂生之花", "嵯峨群峰之翼", "星罗圭壁之晷", "星罗圭璧之晷", "巉岩琢塑之樽", "不动玄石之相",
        "历经风雪的思念", "摧冰而行的执望", "冰雪故园的终期", "遍结寒霜的傲骨", "破冰踏雪的回音",
        "染血的铁之心", "染血的黑之羽", "骑士染血之时", "染血骑士之杯", "染血的铁假面",
        "魔女的炎之花", "魔女常燃之羽", "魔女破灭之时", "魔女的心之火", "焦灼的魔女帽",
        "角斗士的留恋", "角斗士的归宿", "角斗士的希冀", "角斗士的酣醉", "角斗士的凯旋",
        "饰金胸花", "追忆之风", "坚铜罗盘", "沉波之盏", "酒渍船帽",
        "渡火者的决绝", "渡火者的解脱", "渡火者的煎熬", "渡火者的醒悟", "渡火者的智慧",
        "远方的少女之心", "少女飘摇的思念", "少女苦短的良辰", "少女片刻的闲暇", "少女易逝的芳颜",
        "宗室之花", "宗室之翎", "宗室时计", "宗室银瓮", "宗室面具",
        "夏祭之花", "夏祭终末", "夏祭之刻", "夏祭水玉", "夏祭之面",
        "平雷之心", "平雷之羽", "平雷之刻", "平雷之器", "平雷之冠",
        "雷鸟的怜悯", "雷灾的孑遗", "雷霆的时计", "降雷的凶兆", "唤雷的头冠",
        "野花记忆的绿野", "猎人青翠的箭羽", "翠绿猎人的笃定", "翠绿猎人的容器", "翠绿的猎人之冠",
        "乐团的晨光", "琴师的箭羽", "终幕的时计", "终末的时计", "吟游者之壶", "指挥的礼帽",
        "战狂的蔷薇", "战狂的翎羽", "战狂的时计", "战狂的骨杯", "战狂的鬼面",
        "勇士的勋章", "勇士的期许", "勇士的坚毅", "勇士的壮行", "勇士的冠冕",
        "守护之花", "守护徽印", "守护座钟", "守护之皿", "守护束带",
        "流放者之花", "流放者之羽", "流放者怀表", "流放者之杯", "流放者头冠",
        "赌徒的胸花", "赌徒的羽饰", "赌徒的怀表", "赌徒的骰盅", "赌徒的耳环",
        "教官的胸花", "教官的羽饰", "教官的怀表", "教官的茶杯", "教官的帽子",
        "武人的红花", "武人的羽饰", "武人的水漏", "武人的酒杯", "武人的头巾",
        "祭水礼冠", "祭火礼冠", "祭雷礼冠", "祭冰礼冠",
        "故人之心", "归乡之羽", "逐光之石", "异国之盏", "感别之冠",
        "学士的书签", "学士的羽笔", "学士的时钟", "学士的墨杯", "学士的镜片",
        "奇迹之花", "奇迹之羽", "奇迹之沙", "奇迹之杯", "奇迹耳坠",
        "冒险家之花", "冒险家尾羽", "冒险家怀表", "冒险家金杯", "冒险家头带",
        "幸运儿绿花", "幸运儿鹰羽", "幸运儿沙漏", "幸运儿之杯", "幸运儿银冠",
        "游医的银莲", "游医的枭羽", "游医的怀钟", "游医的药壶", "游医的方巾",
        "勋绩之花", "昭武翎羽", "金铜时晷", "盟誓金爵", "将帅兜鍪",
        "无垢之花", "贤医之羽", "停摆之刻", "超越之盏", "嗤笑之面",
        "明威之镡", "切落之羽", "雷云之笼", "绯花之壶", "华饰之兜",
        "羁缠之花", "思忆之矢", "朝露之时", "祈望之心", "无常之面",
    ];

    let mut min_index = 0;
    let mut min_dis = edit_distance::edit_distance(raw, all_artifact_chs[0]);
    let mut same_flag = false;
    for (i, &val) in all_artifact_chs.iter().enumerate().skip(1) {
        let dis = edit_distance::edit_distance(val, raw);
        if dis < min_dis {
            min_dis = dis;
            min_index = i;
            same_flag = false;
        } else if dis == min_dis {
            same_flag = true;
        }
    }

    if same_flag {
        None
    } else {
        Some(String::from(all_artifact_chs[min_index]))
    }
}

impl ArtifactSetName {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSetName> {
        // let s = match get_real_artifact_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        // println!("name: {}", s);
        match s {
            "磐陀裂生之花" => Some(ArtifactSetName::ArchaicPetra),
            "嵯峨群峰之翼" => Some(ArtifactSetName::ArchaicPetra),
            "星罗圭壁之晷" => Some(ArtifactSetName::ArchaicPetra),
            // "壁" is different
            "星罗圭璧之晷" => Some(ArtifactSetName::ArchaicPetra),
            "巉岩琢塑之樽" => Some(ArtifactSetName::ArchaicPetra),
            "不动玄石之相" => Some(ArtifactSetName::ArchaicPetra),
            "历经风雪的思念" => Some(ArtifactSetName::BlizzardStrayer),
            "摧冰而行的执望" => Some(ArtifactSetName::BlizzardStrayer),
            "冰雪故园的终期" => Some(ArtifactSetName::BlizzardStrayer),
            "遍结寒霜的傲骨" => Some(ArtifactSetName::BlizzardStrayer),
            "破冰踏雪的回音" => Some(ArtifactSetName::BlizzardStrayer),
            "染血的铁之心" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血的黑之羽" => Some(ArtifactSetName::BloodstainedChivalry),
            "骑士染血之时" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血骑士之杯" => Some(ArtifactSetName::BloodstainedChivalry),
            "染血的铁假面" => Some(ArtifactSetName::BloodstainedChivalry),
            "魔女的炎之花" => Some(ArtifactSetName::CrimsonWitch),
            "魔女常燃之羽" => Some(ArtifactSetName::CrimsonWitch),
            "魔女破灭之时" => Some(ArtifactSetName::CrimsonWitch),
            "魔女的心之火" => Some(ArtifactSetName::CrimsonWitch),
            "焦灼的魔女帽" => Some(ArtifactSetName::CrimsonWitch),
            "角斗士的留恋" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的归宿" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的希冀" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的酣醉" => Some(ArtifactSetName::GladiatorFinale),
            "角斗士的凯旋" => Some(ArtifactSetName::GladiatorFinale),
            "饰金胸花" => Some(ArtifactSetName::HeartOfDepth),
            "追忆之风" => Some(ArtifactSetName::HeartOfDepth),
            "坚铜罗盘" => Some(ArtifactSetName::HeartOfDepth),
            "沉波之盏" => Some(ArtifactSetName::HeartOfDepth),
            "酒渍船帽" => Some(ArtifactSetName::HeartOfDepth),
            "渡火者的决绝" => Some(ArtifactSetName::LavaWalker),
            "渡火者的解脱" => Some(ArtifactSetName::LavaWalker),
            "渡火者的煎熬" => Some(ArtifactSetName::LavaWalker),
            "渡火者的醒悟" => Some(ArtifactSetName::LavaWalker),
            "渡火者的智慧" => Some(ArtifactSetName::LavaWalker),
            "远方的少女之心" => Some(ArtifactSetName::MaidenBeloved),
            "少女飘摇的思念" => Some(ArtifactSetName::MaidenBeloved),
            "少女苦短的良辰" => Some(ArtifactSetName::MaidenBeloved),
            "少女片刻的闲暇" => Some(ArtifactSetName::MaidenBeloved),
            "少女易逝的芳颜" => Some(ArtifactSetName::MaidenBeloved),
            "宗室之花" => Some(ArtifactSetName::NoblesseOblige),
            "宗室之翎" => Some(ArtifactSetName::NoblesseOblige),
            "宗室时计" => Some(ArtifactSetName::NoblesseOblige),
            "宗室银瓮" => Some(ArtifactSetName::NoblesseOblige),
            "宗室面具" => Some(ArtifactSetName::NoblesseOblige),
            "夏祭之花" => Some(ArtifactSetName::RetracingBolide),
            "夏祭终末" => Some(ArtifactSetName::RetracingBolide),
            "夏祭之刻" => Some(ArtifactSetName::RetracingBolide),
            "夏祭水玉" => Some(ArtifactSetName::RetracingBolide),
            "夏祭之面" => Some(ArtifactSetName::RetracingBolide),
            "平雷之心" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之羽" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之刻" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之器" => Some(ArtifactSetName::ThunderSmoother),
            "平雷之冠" => Some(ArtifactSetName::ThunderSmoother),
            "雷鸟的怜悯" => Some(ArtifactSetName::ThunderingFury),
            "雷灾的孑遗" => Some(ArtifactSetName::ThunderingFury),
            "雷霆的时计" => Some(ArtifactSetName::ThunderingFury),
            "降雷的凶兆" => Some(ArtifactSetName::ThunderingFury),
            "唤雷的头冠" => Some(ArtifactSetName::ThunderingFury),
            "野花记忆的绿野" => Some(ArtifactSetName::ViridescentVenerer),
            "猎人青翠的箭羽" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿猎人的笃定" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿猎人的容器" => Some(ArtifactSetName::ViridescentVenerer),
            "翠绿的猎人之冠" => Some(ArtifactSetName::ViridescentVenerer),
            "乐团的晨光" => Some(ArtifactSetName::WandererTroupe),
            "琴师的箭羽" => Some(ArtifactSetName::WandererTroupe),
            "终幕的时计" => Some(ArtifactSetName::WandererTroupe),
            "终末的时计" => Some(ArtifactSetName::WandererTroupe),
            "吟游者之壶" => Some(ArtifactSetName::WandererTroupe),
            "指挥的礼帽" => Some(ArtifactSetName::WandererTroupe),
            "战狂的蔷薇" => Some(ArtifactSetName::Berserker),
            "战狂的翎羽" => Some(ArtifactSetName::Berserker),
            "战狂的时计" => Some(ArtifactSetName::Berserker),
            "战狂的骨杯" => Some(ArtifactSetName::Berserker),
            "战狂的鬼面" => Some(ArtifactSetName::Berserker),
            "勇士的勋章" => Some(ArtifactSetName::BraveHeart),
            "勇士的期许" => Some(ArtifactSetName::BraveHeart),
            "勇士的坚毅" => Some(ArtifactSetName::BraveHeart),
            "勇士的壮行" => Some(ArtifactSetName::BraveHeart),
            "勇士的冠冕" => Some(ArtifactSetName::BraveHeart),
            "守护之花" => Some(ArtifactSetName::DefenderWill),
            "守护徽印" => Some(ArtifactSetName::DefenderWill),
            "守护座钟" => Some(ArtifactSetName::DefenderWill),
            "守护之皿" => Some(ArtifactSetName::DefenderWill),
            "守护束带" => Some(ArtifactSetName::DefenderWill),
            "流放者之花" => Some(ArtifactSetName::Exile),
            "流放者之羽" => Some(ArtifactSetName::Exile),
            "流放者怀表" => Some(ArtifactSetName::Exile),
            "流放者之杯" => Some(ArtifactSetName::Exile),
            "流放者头冠" => Some(ArtifactSetName::Exile),
            "赌徒的胸花" => Some(ArtifactSetName::Gambler),
            "赌徒的羽饰" => Some(ArtifactSetName::Gambler),
            "赌徒的怀表" => Some(ArtifactSetName::Gambler),
            "赌徒的骰盅" => Some(ArtifactSetName::Gambler),
            "赌徒的耳环" => Some(ArtifactSetName::Gambler),
            "教官的胸花" => Some(ArtifactSetName::Instructor),
            "教官的羽饰" => Some(ArtifactSetName::Instructor),
            "教官的怀表" => Some(ArtifactSetName::Instructor),
            "教官的茶杯" => Some(ArtifactSetName::Instructor),
            "教官的帽子" => Some(ArtifactSetName::Instructor),
            "武人的红花" => Some(ArtifactSetName::MartialArtist),
            "武人的羽饰" => Some(ArtifactSetName::MartialArtist),
            "武人的水漏" => Some(ArtifactSetName::MartialArtist),
            "武人的酒杯" => Some(ArtifactSetName::MartialArtist),
            "武人的头巾" => Some(ArtifactSetName::MartialArtist),
            "祭水礼冠" => Some(ArtifactSetName::PrayersForDestiny),
            "祭火礼冠" => Some(ArtifactSetName::PrayersForIllumination),
            "祭雷礼冠" => Some(ArtifactSetName::PrayersForWisdom),
            "祭冰礼冠" => Some(ArtifactSetName::PrayersToSpringtime),
            "故人之心" => Some(ArtifactSetName::ResolutionOfSojourner),
            "归乡之羽" => Some(ArtifactSetName::ResolutionOfSojourner),
            "逐光之石" => Some(ArtifactSetName::ResolutionOfSojourner),
            "异国之盏" => Some(ArtifactSetName::ResolutionOfSojourner),
            "感别之冠" => Some(ArtifactSetName::ResolutionOfSojourner),
            "学士的书签" => Some(ArtifactSetName::Scholar),
            "学士的羽笔" => Some(ArtifactSetName::Scholar),
            "学士的时钟" => Some(ArtifactSetName::Scholar),
            "学士的墨杯" => Some(ArtifactSetName::Scholar),
            "学士的镜片" => Some(ArtifactSetName::Scholar),
            "奇迹之花" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之羽" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之沙" => Some(ArtifactSetName::TinyMiracle),
            "奇迹之杯" => Some(ArtifactSetName::TinyMiracle),
            "奇迹耳坠" => Some(ArtifactSetName::TinyMiracle),
            "冒险家之花" => Some(ArtifactSetName::Adventurer),
            "冒险家尾羽" => Some(ArtifactSetName::Adventurer),
            "冒险家怀表" => Some(ArtifactSetName::Adventurer),
            "冒险家金杯" => Some(ArtifactSetName::Adventurer),
            "冒险家头带" => Some(ArtifactSetName::Adventurer),
            "幸运儿绿花" => Some(ArtifactSetName::LuckyDog),
            "幸运儿鹰羽" => Some(ArtifactSetName::LuckyDog),
            "幸运儿沙漏" => Some(ArtifactSetName::LuckyDog),
            "幸运儿之杯" => Some(ArtifactSetName::LuckyDog),
            "幸运儿银冠" => Some(ArtifactSetName::LuckyDog),
            "游医的银莲" => Some(ArtifactSetName::TravelingDoctor),
            "游医的枭羽" => Some(ArtifactSetName::TravelingDoctor),
            "游医的怀钟" => Some(ArtifactSetName::TravelingDoctor),
            "游医的药壶" => Some(ArtifactSetName::TravelingDoctor),
            "游医的方巾" => Some(ArtifactSetName::TravelingDoctor),
            "勋绩之花" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "昭武翎羽" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "金铜时晷" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "盟誓金爵" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "将帅兜鍪" => Some(ArtifactSetName::TenacityOfTheMillelith),
            "无垢之花" => Some(ArtifactSetName::PaleFlame),
            "贤医之羽" => Some(ArtifactSetName::PaleFlame),
            "停摆之刻" => Some(ArtifactSetName::PaleFlame),
            "超越之盏" => Some(ArtifactSetName::PaleFlame),
            "嗤笑之面" => Some(ArtifactSetName::PaleFlame),
            "明威之镡" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "切落之羽" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "雷云之笼" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "绯花之壶" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "华饰之兜" => Some(ArtifactSetName::EmblemOfSeveredFate),
            "羁缠之花" => Some(ArtifactSetName::ShimenawaReminiscence),
            "思忆之矢" => Some(ArtifactSetName::ShimenawaReminiscence),
            "朝露之时" => Some(ArtifactSetName::ShimenawaReminiscence),
            "祈望之心" => Some(ArtifactSetName::ShimenawaReminiscence),
            "无常之面" => Some(ArtifactSetName::ShimenawaReminiscence),
            _ => None,
        }
    }
}

impl ArtifactSlot {
    pub fn from_zh_cn(s: &str) -> Option<ArtifactSlot> {
        // let s = match get_real_artifact_name_chs(s) {
        //     Some(v) => v,
        //     None => return None,
        // };
        match s {
            "磐陀裂生之花" => Some(ArtifactSlot::Flower),
            "嵯峨群峰之翼" => Some(ArtifactSlot::Feather),
            "星罗圭壁之晷" => Some(ArtifactSlot::Sand),
            "星罗圭璧之晷" => Some(ArtifactSlot::Sand),
            "巉岩琢塑之樽" => Some(ArtifactSlot::Goblet),
            "不动玄石之相" => Some(ArtifactSlot::Head),
            "历经风雪的思念" => Some(ArtifactSlot::Flower),
            "摧冰而行的执望" => Some(ArtifactSlot::Feather),
            "冰雪故园的终期" => Some(ArtifactSlot::Sand),
            "遍结寒霜的傲骨" => Some(ArtifactSlot::Goblet),
            "破冰踏雪的回音" => Some(ArtifactSlot::Head),
            "染血的铁之心" => Some(ArtifactSlot::Flower),
            "染血的黑之羽" => Some(ArtifactSlot::Feather),
            "骑士染血之时" => Some(ArtifactSlot::Sand),
            "染血骑士之杯" => Some(ArtifactSlot::Goblet),
            "染血的铁假面" => Some(ArtifactSlot::Head),
            "魔女的炎之花" => Some(ArtifactSlot::Flower),
            "魔女常燃之羽" => Some(ArtifactSlot::Feather),
            "魔女破灭之时" => Some(ArtifactSlot::Sand),
            "魔女的心之火" => Some(ArtifactSlot::Goblet),
            "焦灼的魔女帽" => Some(ArtifactSlot::Head),
            "角斗士的留恋" => Some(ArtifactSlot::Flower),
            "角斗士的归宿" => Some(ArtifactSlot::Feather),
            "角斗士的希冀" => Some(ArtifactSlot::Sand),
            "角斗士的酣醉" => Some(ArtifactSlot::Goblet),
            "角斗士的凯旋" => Some(ArtifactSlot::Head),
            "饰金胸花" => Some(ArtifactSlot::Flower),
            "追忆之风" => Some(ArtifactSlot::Feather),
            "坚铜罗盘" => Some(ArtifactSlot::Sand),
            "沉波之盏" => Some(ArtifactSlot::Goblet),
            "酒渍船帽" => Some(ArtifactSlot::Head),
            "渡火者的决绝" => Some(ArtifactSlot::Flower),
            "渡火者的解脱" => Some(ArtifactSlot::Feather),
            "渡火者的煎熬" => Some(ArtifactSlot::Sand),
            "渡火者的醒悟" => Some(ArtifactSlot::Goblet),
            "渡火者的智慧" => Some(ArtifactSlot::Head),
            "远方的少女之心" => Some(ArtifactSlot::Flower),
            "少女飘摇的思念" => Some(ArtifactSlot::Feather),
            "少女苦短的良辰" => Some(ArtifactSlot::Sand),
            "少女片刻的闲暇" => Some(ArtifactSlot::Goblet),
            "少女易逝的芳颜" => Some(ArtifactSlot::Head),
            "宗室之花" => Some(ArtifactSlot::Flower),
            "宗室之翎" => Some(ArtifactSlot::Feather),
            "宗室时计" => Some(ArtifactSlot::Sand),
            "宗室银瓮" => Some(ArtifactSlot::Goblet),
            "宗室面具" => Some(ArtifactSlot::Head),
            "夏祭之花" => Some(ArtifactSlot::Flower),
            "夏祭终末" => Some(ArtifactSlot::Feather),
            "夏祭之刻" => Some(ArtifactSlot::Sand),
            "夏祭水玉" => Some(ArtifactSlot::Goblet),
            "夏祭之面" => Some(ArtifactSlot::Head),
            "平雷之心" => Some(ArtifactSlot::Flower),
            "平雷之羽" => Some(ArtifactSlot::Feather),
            "平雷之刻" => Some(ArtifactSlot::Sand),
            "平雷之器" => Some(ArtifactSlot::Goblet),
            "平雷之冠" => Some(ArtifactSlot::Head),
            "雷鸟的怜悯" => Some(ArtifactSlot::Flower),
            "雷灾的孑遗" => Some(ArtifactSlot::Feather),
            "雷霆的时计" => Some(ArtifactSlot::Sand),
            "降雷的凶兆" => Some(ArtifactSlot::Goblet),
            "唤雷的头冠" => Some(ArtifactSlot::Head),
            "野花记忆的绿野" => Some(ArtifactSlot::Flower),
            "猎人青翠的箭羽" => Some(ArtifactSlot::Feather),
            "翠绿猎人的笃定" => Some(ArtifactSlot::Sand),
            "翠绿猎人的容器" => Some(ArtifactSlot::Goblet),
            "翠绿的猎人之冠" => Some(ArtifactSlot::Head),
            "乐团的晨光" => Some(ArtifactSlot::Flower),
            "琴师的箭羽" => Some(ArtifactSlot::Feather),
            "终幕的时计" => Some(ArtifactSlot::Sand),
            "终末的时计" => Some(ArtifactSlot::Sand),
            "吟游者之壶" => Some(ArtifactSlot::Goblet),
            "指挥的礼帽" => Some(ArtifactSlot::Head),
            "战狂的蔷薇" => Some(ArtifactSlot::Flower),
            "战狂的翎羽" => Some(ArtifactSlot::Feather),
            "战狂的时计" => Some(ArtifactSlot::Sand),
            "战狂的骨杯" => Some(ArtifactSlot::Goblet),
            "战狂的鬼面" => Some(ArtifactSlot::Head),
            "勇士的勋章" => Some(ArtifactSlot::Flower),
            "勇士的期许" => Some(ArtifactSlot::Feather),
            "勇士的坚毅" => Some(ArtifactSlot::Sand),
            "勇士的壮行" => Some(ArtifactSlot::Goblet),
            "勇士的冠冕" => Some(ArtifactSlot::Head),
            "守护之花" => Some(ArtifactSlot::Flower),
            "守护徽印" => Some(ArtifactSlot::Feather),
            "守护座钟" => Some(ArtifactSlot::Sand),
            "守护之皿" => Some(ArtifactSlot::Goblet),
            "守护束带" => Some(ArtifactSlot::Head),
            "流放者之花" => Some(ArtifactSlot::Flower),
            "流放者之羽" => Some(ArtifactSlot::Feather),
            "流放者怀表" => Some(ArtifactSlot::Sand),
            "流放者之杯" => Some(ArtifactSlot::Goblet),
            "流放者头冠" => Some(ArtifactSlot::Head),
            "赌徒的胸花" => Some(ArtifactSlot::Flower),
            "赌徒的羽饰" => Some(ArtifactSlot::Feather),
            "赌徒的怀表" => Some(ArtifactSlot::Sand),
            "赌徒的骰盅" => Some(ArtifactSlot::Goblet),
            "赌徒的耳环" => Some(ArtifactSlot::Head),
            "教官的胸花" => Some(ArtifactSlot::Flower),
            "教官的羽饰" => Some(ArtifactSlot::Feather),
            "教官的怀表" => Some(ArtifactSlot::Sand),
            "教官的茶杯" => Some(ArtifactSlot::Goblet),
            "教官的帽子" => Some(ArtifactSlot::Head),
            "武人的红花" => Some(ArtifactSlot::Flower),
            "武人的羽饰" => Some(ArtifactSlot::Feather),
            "武人的水漏" => Some(ArtifactSlot::Sand),
            "武人的酒杯" => Some(ArtifactSlot::Goblet),
            "武人的头巾" => Some(ArtifactSlot::Head),
            "祭水礼冠" => Some(ArtifactSlot::Head),
            "祭火礼冠" => Some(ArtifactSlot::Head),
            "祭雷礼冠" => Some(ArtifactSlot::Head),
            "祭冰礼冠" => Some(ArtifactSlot::Head),
            "故人之心" => Some(ArtifactSlot::Flower),
            "归乡之羽" => Some(ArtifactSlot::Feather),
            "逐光之石" => Some(ArtifactSlot::Sand),
            "异国之盏" => Some(ArtifactSlot::Goblet),
            "感别之冠" => Some(ArtifactSlot::Head),
            "学士的书签" => Some(ArtifactSlot::Flower),
            "学士的羽笔" => Some(ArtifactSlot::Feather),
            "学士的时钟" => Some(ArtifactSlot::Sand),
            "学士的墨杯" => Some(ArtifactSlot::Goblet),
            "学士的镜片" => Some(ArtifactSlot::Head),
            "奇迹之花" => Some(ArtifactSlot::Flower),
            "奇迹之羽" => Some(ArtifactSlot::Feather),
            "奇迹之沙" => Some(ArtifactSlot::Sand),
            "奇迹之杯" => Some(ArtifactSlot::Goblet),
            "奇迹耳坠" => Some(ArtifactSlot::Head),
            "冒险家之花" => Some(ArtifactSlot::Flower),
            "冒险家尾羽" => Some(ArtifactSlot::Feather),
            "冒险家怀表" => Some(ArtifactSlot::Sand),
            "冒险家金杯" => Some(ArtifactSlot::Goblet),
            "冒险家头带" => Some(ArtifactSlot::Head),
            "幸运儿绿花" => Some(ArtifactSlot::Flower),
            "幸运儿鹰羽" => Some(ArtifactSlot::Feather),
            "幸运儿沙漏" => Some(ArtifactSlot::Sand),
            "幸运儿之杯" => Some(ArtifactSlot::Goblet),
            "幸运儿银冠" => Some(ArtifactSlot::Head),
            "游医的银莲" => Some(ArtifactSlot::Flower),
            "游医的枭羽" => Some(ArtifactSlot::Feather),
            "游医的怀钟" => Some(ArtifactSlot::Sand),
            "游医的药壶" => Some(ArtifactSlot::Goblet),
            "游医的方巾" => Some(ArtifactSlot::Head),
            "勋绩之花" => Some(ArtifactSlot::Flower),
            "昭武翎羽" => Some(ArtifactSlot::Feather),
            "金铜时晷" => Some(ArtifactSlot::Sand),
            "盟誓金爵" => Some(ArtifactSlot::Goblet),
            "将帅兜鍪" => Some(ArtifactSlot::Head),
            "无垢之花" => Some(ArtifactSlot::Flower),
            "贤医之羽" => Some(ArtifactSlot::Feather),
            "停摆之刻" => Some(ArtifactSlot::Sand),
            "超越之盏" => Some(ArtifactSlot::Goblet),
            "嗤笑之面" => Some(ArtifactSlot::Head),
            "明威之镡" => Some(ArtifactSlot::Flower),
            "切落之羽" => Some(ArtifactSlot::Feather),
            "雷云之笼" => Some(ArtifactSlot::Sand),
            "绯花之壶" => Some(ArtifactSlot::Goblet),
            "华饰之兜" => Some(ArtifactSlot::Head),
            "羁缠之花" => Some(ArtifactSlot::Flower),
            "思忆之矢" => Some(ArtifactSlot::Feather),
            "朝露之时" => Some(ArtifactSlot::Sand),
            "祈望之心" => Some(ArtifactSlot::Goblet),
            "无常之面" => Some(ArtifactSlot::Head),
            _ => None,
        }
    }
}
