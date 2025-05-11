use eframe::egui::{
    self, CentralPanel, Context, FontData, FontDefinitions, FontFamily, RichText, Ui,
};
use eframe::{App, CreationContext};

#[derive(PartialEq)]
enum Tab {
    ShowFrets,
    Settings,
    Info,
}

pub struct FretMeNotApp {
    current_tab: Tab,
}

impl Default for FretMeNotApp {
    fn default() -> Self {
        Self {
            current_tab: Tab::ShowFrets,
        }
    }
}

impl App for FretMeNotApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // 제목
            ui.heading("🎸 FretMeNot 🎸");
            ui.label("기타자판을 외워봅시다!!");

            // 탭
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.current_tab == Tab::ShowFrets, "🎵 지판 확인")
                    .clicked()
                {
                    self.current_tab = Tab::ShowFrets;
                }

                if ui
                    .selectable_label(self.current_tab == Tab::Settings, "⚙ 설정")
                    .clicked()
                {
                    self.current_tab = Tab::Settings;
                }

                if ui
                    .selectable_label(self.current_tab == Tab::Info, "👍 정보")
                    .clicked()
                {
                    self.current_tab = Tab::Info;
                }
            });

            ui.separator();

            // 선택된 탭에 따라 내용 표시
            match self.current_tab {
                Tab::ShowFrets => tab_show_frets(ui),
                Tab::Settings => tab_settings(ui),
                Tab::Info => tab_info(ui),
            }
        });
    }
}

// 프렛보드 간단히 표시 (예제용)
//fn TabShowFrets(ui: &mut Ui) {
    //ui.label(RichText::new("기타 지판").strong());

//    for string in (1..=6).rev() {
//        ui.horizontal(|ui| {
//            for fret in 0..=5 {
//                ui.label(format!("{}-{}", string, fret));
//            }
//        });
//    }

fn tab_show_frets(ui: &mut Ui) {
    let strings = ["1번줄", "2번줄", "3번줄", "4번줄", "5번줄", "6번줄"];
    let frets = 0..=24;

    egui::Grid::new("fretboard")
        .striped(true)
        .spacing([4.0, 8.0])
        .show(ui, |ui| {
            // 헤더 (프렛 번호)
            ui.label(" "); // 왼쪽 상단 빈칸
            for fret in frets.clone() {
                ui.label(format!("{}", fret));
            }
            ui.end_row();

            // 줄별 프렛
            for string in strings.iter() {
                ui.label(*string);
                for _fret in frets.clone() {
                    // TODO: 여기서 음 이름을 계산해 넣을 수 있음
                    ui.label("●"); // 또는 "C", "F#" 등
                }
                ui.end_row();
            }
        });
}

fn tab_settings(ui: &mut Ui) {
    ui.label("설정 탭입니다.");
    ui.label("Setting Tab.");
}

fn tab_info(ui: &mut Ui) {
    ui.label(RichText::new("👤 제작자: 잼니크"));
    ui.label(RichText::new("📧 이메일: gam1532@gmail.com"));
    ui.hyperlink_to("🔗 GitHub: https://github.com/JeaMinLim", "https://github.com/JeaMinLim");
}

// 폰트 설정 함수
pub fn setup_fonts(cc: &CreationContext) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Regular".to_owned(),
       //FontData::from_owned(std::fs::read("fonts/NanumGothic-Regular.ttf").unwrap()),
       FontData::from_static(include_bytes!("../fonts/NanumGothic-Regular.ttf")),
    );
    fonts.font_data.insert(
        "Bold".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NanumGothic-Bold.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NanumGothic-Bold.ttf")),
    );
    fonts.font_data.insert(
        "ExtraBold".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NanumGothic-ExtraBold.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NanumGothic-ExtraBold.ttf")),
    );
    fonts.font_data.insert(
        "Emoji".to_owned(),
        //FontData::from_owned(std::fs::read("fonts/NotoEmoji-VariableFont_wght.ttf").unwrap()),
        FontData::from_static(include_bytes!("../fonts/NotoEmoji-VariableFont_wght.ttf")),
    );

    // Proportional에 Emoji -> Regular 순서로 등록
    //if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        //family.clear();
        //family.push("Emoji".to_owned());
        //family.push("Regular".to_owned());
    //}

    // 사용자 정의 굵기 등록
    //fonts
        //.families
        //.insert(FontFamily::Name("Bold".into()), vec!["Bold".to_owned()]);
    //fonts
        //.families
        //.insert(FontFamily::Name("ExtraBold".into()), vec!["ExtraBold".to_owned()]);

    fonts  
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Regular".to_owned());

    cc.egui_ctx.set_fonts(fonts);
    
}
