use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrcProfile {
    pub id: String,
    pub version: i64,
    pub name: String,
    pub last_used_at: String,
    pub artcc_id: String,
    pub last_used_environment: String,
    pub last_used_position_id: String,
    pub network_rating: String,
    pub role: String,
    pub controller_info: String,
    pub display_window_settings: Vec<DisplayWindowSettings>,
    pub controller_list_settings: InformationWindowSettings,
    pub flight_plan_editor_settings: InformationWindowSettings,
    pub messages_area_settings: InformationWindowSettings,
    pub voice_switch_settings: InformationWindowSettings,
    pub bookmarks: Vec<Value>,
    pub selected_beacon_codes: Vec<Value>,
    pub invert_numeric_keypad: Option<bool>,
    pub secondary_voice_switch_position_ids: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayWindowSettings {
    pub id: String,
    pub window_settings: WindowSettings,
    pub cover_task_bar_when_maximized: bool,
    pub display_settings: Vec<DisplaySettings>,
    pub selected_display_id: String,
    pub bookmarks: Vec<Bookmark2>,
    pub use_global_bookmarks: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WindowSettings {
    pub is_visible: bool,
    pub bounds: String,
    pub scale_factor: f64,
    pub is_maximized: bool,
    pub show_title_bar: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplaySettings {
    #[serde(rename = "$type")]
    pub type_field: String,
    pub center: Option<Point>,
    pub range: Option<f64>,
    pub bookmarks: Vec<Bookmark>,
    pub top_down_mode_enabled: Option<bool>,
    pub ground_target_leader_length: Option<i64>,
    pub ground_target_leader_direction: Option<String>,
    pub area_id: Option<String>,
    pub position_id: Option<String>,
    pub is_dcb_visible: Option<bool>,
    pub sign_on_list_show_all: Option<bool>,
    pub display_winds_in_ssa: Option<bool>,
    pub current_pref_set: Option<CurrentPrefSet>,
    pub id: String,
    pub facility_id: String,
    pub invert_numeric_keypad: Option<bool>,
    pub disable_mouse_pan_zoom: bool,
    pub night_mode: Option<bool>,
    pub volume: Option<i64>,
    pub dcb_off: Option<bool>,
    pub active_position_ids: Option<Vec<String>>,
    pub show_dcb: Option<bool>,
    pub status_text_font_size: Option<i64>,
    pub show_status_text: Option<bool>,
    pub show_full_metar: Option<bool>,
    pub status_text_at_top: Option<bool>,
    pub operating_initials: Option<String>,
    pub rotation: Option<f64>,
    pub data_block_font_size: Option<i64>,
    pub show_data_blocks: Option<bool>,
    pub callsign_display: Option<String>,
    pub show_type_code_and_altitude: Option<bool>,
    pub leader_length: Option<i64>,
    pub leader_direction: Option<String>,
    pub background_image_brightness: Option<i64>,
    pub show_background_image: Option<bool>,
    pub show_high_res_background_image: Option<bool>,
    pub show_airport_diagram: Option<bool>,
    pub background_color: Option<String>,
    pub data_block_color: Option<String>,
    pub aircraft_color: Option<String>,
    pub status_text_color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bookmark {
    pub index: i64,
    pub center: Point,
    pub range: f64,
    pub top_down_mode_enabled: Option<bool>,
    pub rotation: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPrefSet {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Version")]
    pub version: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Range")]
    pub range: Option<f64>,
    #[serde(rename = "DisplayCenter")]
    pub display_center: Option<Point>,
    #[serde(rename = "DisplayOffCenter")]
    pub display_off_center: Option<bool>,
    #[serde(rename = "RangeRingSpacing")]
    pub range_ring_spacing: Option<i64>,
    #[serde(rename = "RangeRingCenter")]
    pub range_ring_center: Option<Point>,
    #[serde(rename = "RangeRingsOffCenter")]
    pub range_rings_off_center: Option<bool>,
    #[serde(rename = "LeaderDirTracked")]
    pub leader_dir_tracked: Option<String>,
    #[serde(rename = "LeaderDirAssociated")]
    pub leader_dir_associated: Option<String>,
    #[serde(rename = "LeaderDirUnassociated")]
    pub leader_dir_unassociated: Option<String>,
    #[serde(rename = "LeaderLength")]
    pub leader_length: Option<i64>,
    #[serde(rename = "HistoryCount")]
    pub history_count: Option<i64>,
    #[serde(rename = "PtlLength")]
    pub ptl_length: Option<f64>,
    #[serde(rename = "PtlOwn")]
    pub ptl_own: Option<bool>,
    #[serde(rename = "PtlAll")]
    pub ptl_all: Option<bool>,
    #[serde(rename = "VisibleLists")]
    pub visible_lists: Option<String>,
    #[serde(rename = "SelectedVideoMapListType")]
    pub selected_video_map_list_type: Option<String>,
    #[serde(rename = "VisibleSsaFields")]
    pub visible_ssa_fields: Option<String>,
    #[serde(rename = "AllSsaFieldsVisible")]
    pub all_ssa_fields_visible: Option<bool>,
    #[serde(rename = "VisibleGiTextLines")]
    pub visible_gi_text_lines: Option<String>,
    #[serde(rename = "BrightnessDcb")]
    pub brightness_dcb: Option<i64>,
    #[serde(rename = "BrightnessMpa")]
    pub brightness_mpa: Option<i64>,
    #[serde(rename = "BrightnessMpb")]
    pub brightness_mpb: Option<i64>,
    #[serde(rename = "BrightnessFdb")]
    pub brightness_fdb: Option<i64>,
    #[serde(rename = "BrightnessLst")]
    pub brightness_lst: Option<i64>,
    #[serde(rename = "BrightnessPos")]
    pub brightness_pos: Option<i64>,
    #[serde(rename = "BrightnessLdb")]
    pub brightness_ldb: Option<i64>,
    #[serde(rename = "BrightnessOth")]
    pub brightness_oth: Option<i64>,
    #[serde(rename = "BrightnessTls")]
    pub brightness_tls: Option<i64>,
    #[serde(rename = "BrightnessRr")]
    pub brightness_rr: Option<i64>,
    #[serde(rename = "BrightnessCmp")]
    pub brightness_cmp: Option<i64>,
    #[serde(rename = "BrightnessBcn")]
    pub brightness_bcn: Option<i64>,
    #[serde(rename = "BrightnessPri")]
    pub brightness_pri: Option<i64>,
    #[serde(rename = "BrightnessHst")]
    pub brightness_hst: Option<i64>,
    #[serde(rename = "CharSizeDataBlocks")]
    pub char_size_data_blocks: Option<i64>,
    #[serde(rename = "CharSizeLists")]
    pub char_size_lists: Option<i64>,
    #[serde(rename = "CharSizeDcb")]
    pub char_size_dcb: Option<i64>,
    #[serde(rename = "CharSizeTools")]
    pub char_size_tools: Option<i64>,
    #[serde(rename = "CharSizePositionSymbols")]
    pub char_size_position_symbols: Option<i64>,
    #[serde(rename = "PreviewAreaLocation")]
    pub preview_area_location: Value,
    #[serde(rename = "SsaLocation")]
    pub ssa_location: Option<String>,
    #[serde(rename = "TabListLocation")]
    pub tab_list_location: Option<String>,
    #[serde(rename = "VfrListLocation")]
    pub vfr_list_location: Option<String>,
    #[serde(rename = "LaCaMciListLocation")]
    pub la_ca_mci_list_location: Option<String>,
    #[serde(rename = "CoastSuspendListLocation")]
    pub coast_suspend_list_location: Option<String>,
    #[serde(rename = "SignOnListLocation")]
    pub sign_on_list_location: Option<String>,
    #[serde(rename = "VideoMapListLocation")]
    pub video_map_list_location: Option<String>,
    #[serde(rename = "CrdaListLocation")]
    pub crda_list_location: Option<String>,
    #[serde(rename = "TowerList1Location")]
    pub tower_list1location: Option<String>,
    #[serde(rename = "TowerList2Location")]
    pub tower_list2location: Option<String>,
    #[serde(rename = "TowerList3Location")]
    pub tower_list3location: Option<String>,
    #[serde(rename = "TabListSize")]
    pub tab_list_size: Option<i64>,
    #[serde(rename = "CoastSuspendListSize")]
    pub coast_suspend_list_size: Option<i64>,
    #[serde(rename = "VfrListSize")]
    pub vfr_list_size: Option<i64>,
    #[serde(rename = "TowerList1Size")]
    pub tower_list1size: Option<i64>,
    #[serde(rename = "TowerList2Size")]
    pub tower_list2size: Option<i64>,
    #[serde(rename = "TowerList3Size")]
    pub tower_list3size: Option<i64>,
    #[serde(rename = "SelectedVideoMapIds")]
    #[serde(default)]
    pub selected_video_map_ids: Vec<i64>,
    #[serde(rename = "AltitudeFilterUnassociated")]
    pub altitude_filter_unassociated: Option<AltitudeFilter>,
    #[serde(rename = "AltitudeFilterAssociated")]
    pub altitude_filter_associated: Option<AltitudeFilter>,
    #[serde(rename = "QuickLookedTcps")]
    #[serde(default)]
    pub quick_looked_tcps: Vec<Value>,
    #[serde(rename = "QuickLookAll")]
    pub quick_look_all: Option<bool>,
    #[serde(rename = "SelectedBeaconCodes")]
    #[serde(default)]
    pub selected_beacon_codes: Vec<Value>,
    #[serde(rename = "DcbLocation")]
    pub dcb_location: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Windows")]
    pub windows: Option<Vec<Window>>,
    #[serde(rename = "ShowCoastList")]
    pub show_coast_list: Option<bool>,
    #[serde(rename = "CoastListLocation")]
    pub coast_list_location: Option<Margins>,
    #[serde(rename = "DcbPosition")]
    pub dcb_position: Option<String>,
    #[serde(rename = "ShowAltitudeInDb")]
    pub show_altitude_in_db: Option<bool>,
    #[serde(rename = "ShowAircraftTypeInDb")]
    pub show_aircraft_type_in_db: Option<bool>,
    #[serde(rename = "ShowSensorsInDb")]
    pub show_sensors_in_db: Option<bool>,
    #[serde(rename = "ShowAircraftCategoryInDb")]
    pub show_aircraft_category_in_db: Option<bool>,
    #[serde(rename = "ShowFixInDb")]
    pub show_fix_in_db: Option<bool>,
    #[serde(rename = "ShowVelocityInDb")]
    pub show_velocity_in_db: Option<bool>,
    #[serde(rename = "ShowScratchpadsInDb")]
    pub show_scratchpads_in_db: Option<bool>,
    #[serde(rename = "AlertMessageLocation")]
    pub alert_message_location: Option<Margins>,
    #[serde(rename = "VectorLength")]
    pub vector_length: Option<i64>,
    #[serde(rename = "ListsBrightness")]
    pub lists_brightness: Option<i64>,
    #[serde(rename = "DcbBrightness")]
    pub dcb_brightness: Option<i64>,
    #[serde(rename = "DcbCharSize")]
    pub dcb_char_size: Option<i64>,
    #[serde(rename = "CoastSuspendCharSize")]
    pub coast_suspend_char_size: Option<i64>,
    #[serde(rename = "PreviewAreaCharSize")]
    pub preview_area_char_size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AltitudeFilter {
    pub low: i64,
    pub high: i64,
    pub is_valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "DisplayType")]
    pub display_type: String,
    #[serde(rename = "Bounds")]
    pub bounds: Bounds,
    #[serde(rename = "Center")]
    pub center: Point,
    #[serde(rename = "Range")]
    pub range: f64,
    #[serde(rename = "Rotation")]
    pub rotation: f64,
    #[serde(rename = "EnableAntiAliasing")]
    pub enable_anti_aliasing: bool,
    #[serde(rename = "BackgroundBrightness")]
    pub background_brightness: i64,
    #[serde(rename = "HoldBarsBrightness")]
    pub hold_bars_brightness: i64,
    #[serde(rename = "MovementAreasBrightness")]
    pub movement_areas_brightness: i64,
    #[serde(rename = "TrackBrightness")]
    pub track_brightness: i64,
    #[serde(rename = "DataBlocksBrightness")]
    pub data_blocks_brightness: i64,
    #[serde(rename = "TempMapAreasBrightness")]
    pub temp_map_areas_brightness: i64,
    #[serde(rename = "TempMapTextBrightness")]
    pub temp_map_text_brightness: i64,
    #[serde(rename = "DataBlockTraitAreas")]
    pub data_block_trait_areas: Vec<DataBlockTraitArea>,
    #[serde(rename = "ShowDataBlocks")]
    pub show_data_blocks: bool,
    #[serde(rename = "FullDataBlocks")]
    pub full_data_blocks: bool,
    #[serde(rename = "DataBlockCharSize")]
    pub data_block_char_size: i64,
    #[serde(rename = "TempDataCharSize")]
    pub temp_data_char_size: i64,
    #[serde(rename = "ShowHistory")]
    pub show_history: bool,
    #[serde(rename = "HistoryLength")]
    pub history_length: i64,
    #[serde(rename = "LeaderDirection")]
    pub leader_direction: String,
    #[serde(rename = "LeaderLength")]
    pub leader_length: i64,
    #[serde(rename = "ShowVectorLine")]
    pub show_vector_line: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bounds {
    pub location: Margins,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataBlockTraitArea {
    pub area: Area,
    pub traits: Traits,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Area {
    pub points: Vec<Point>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Traits {
    pub data_blocks_off: bool,
    pub full_data_blocks: bool,
    pub show_altitude: bool,
    pub show_aircraft_type: bool,
    pub show_sensors: bool,
    pub show_aircraft_category: bool,
    pub show_fix: bool,
    pub show_velocity: bool,
    pub show_scratchpads: bool,
    pub data_blocks_char_size: i64,
    pub data_blocks_brightness: i64,
    pub show_vector: bool,
    pub leader_length: i64,
    pub leader_direction: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Margins {
    pub left_margin: Option<i16>,
    pub right_margin: Option<i16>,
    pub top_margin: Option<i16>,
    pub bottom_margin: Option<i16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bookmark2 {
    pub index: i64,
    pub selected_display_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InformationWindowSettings {
    pub type_field: String,
    pub window_settings: WindowSettings,
}
