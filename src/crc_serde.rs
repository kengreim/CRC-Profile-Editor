use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct CrcProfile {
    pub id: String,
    pub version: i32,
    pub name: String,
    pub last_used_at: String,
    pub artcc_id: String,
    pub last_used_environment: Option<String>,
    pub last_used_position_id: Option<String>,
    pub network_rating: String,
    pub role: String,
    pub controller_info: String,
    pub display_window_settings: Vec<DisplayWindowSettings>,
    pub controller_list_settings: InformationWindowSettings,
    pub flight_plan_editor_settings: InformationWindowSettings,
    pub messages_area_settings: InformationWindowSettings,
    pub voice_switch_settings: Option<InformationWindowSettings>, // Option to deal with old profiles
    pub bookmarks: Vec<GlobalBookmark>,
    pub selected_beacon_codes: Vec<String>,
    pub invert_numeric_keypad: Option<bool>,
    pub secondary_voice_switch_position_ids: Option<Vec<String>>, // Option to deal with old profiles
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DisplayWindowSettings {
    pub id: String,
    pub window_settings: WindowSettings,
    pub cover_task_bar_when_maximized: bool,
    pub display_settings: Vec<DisplaySettings>,
    pub selected_display_id: String,
    pub bookmarks: Vec<DisplayWindowBookmark>,
    pub use_global_bookmarks: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct WindowSettings {
    pub is_visible: bool,
    pub bounds: String,
    pub scale_factor: f64,
    pub is_maximized: bool,
    pub show_title_bar: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct AsdexDisplaySettings {
    pub bookmarks: Vec<DisplayBookmark>,
    pub night_mode: bool,
    pub volume: i32,
    pub dcb_off: bool,
    pub active_position_ids: Vec<String>,
    pub show_dcb: bool,
    pub status_text_font_size: i32,
    pub show_status_text: bool,
    pub show_full_metar: bool,
    pub status_text_at_top: bool,
    pub operating_initials: String,
    pub current_pref_set: AsdexPrefSet,
    pub id: String,
    pub facility_id: String,
    pub invert_numeric_keypad: Option<bool>,
    pub disable_mouse_pan_zoom: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct TowerCabDisplaySettings {
    pub center: Point,
    pub range: f64,
    pub rotation: f64,
    pub bookmarks: Vec<DisplayBookmark>,
    pub data_block_font_size: i32,
    pub show_data_blocks: bool,
    pub callsign_display: String,
    pub show_type_code_and_altitude: bool,
    pub leader_length: i32,
    pub leader_direction: String,
    pub status_text_font_size: i32,
    pub show_status_text: bool,
    pub show_full_metar: bool,
    pub status_text_at_top: bool,
    pub background_image_brightness: i32,
    pub show_background_image: bool,
    pub show_high_res_background_image: bool,
    pub show_airport_diagram: bool,
    pub background_color: String,
    pub data_block_color: String,
    pub aircraft_color: String,
    pub status_text_color: String,
    pub id: String,
    pub facility_id: String,
    pub invert_numeric_keypad: Option<bool>,
    pub disable_mouse_pan_zoom: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct StarsDisplaySettings {
    pub center: Option<Point>,
    pub range: f64,
    pub bookmarks: Vec<DisplayBookmark>,
    pub top_down_mode_enabled: bool,
    pub ground_target_leader_length: i32,
    pub ground_target_leader_direction: String,
    pub area_id: String,
    pub position_id: String,
    pub is_dcb_visible: bool,
    pub sign_on_list_show_all: bool,
    pub display_winds_in_ssa: bool,
    pub current_pref_set: StarsPrefSet,
    pub id: String,
    pub facility_id: String,
    pub invert_numeric_keypad: Option<bool>,
    pub disable_mouse_pan_zoom: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct EramDisplaySettings {
    pub center: Point,
    pub top_down_mode_enabled: bool,
    pub ground_target_leader_length: i32,
    pub ground_target_leader_direction: String,
    pub range: f64,
    pub bookmarks: Vec<DisplayBookmark>,
    pub raise_master_toolbar: bool,
    pub bcgs: Bcgs,
    pub bcg_modifiers: BcgModifiers,
    pub font_sizes: FontSizes,
    pub cursor_size: i32,
    pub declutter_level: i32,
    pub nexrad_levels: i32,
    pub time_view_settings: TimeViewSettings,
    pub mca_view_settings: McaViewSettings,
    pub response_area_view_settings: ResponseAreaViewSettings,
    pub altimeter_settings_view_settings: AltimeterSettingsViewSettings,
    pub beacon_code_view_settings: BeaconCodeViewSettings,
    pub weather_station_report_view_settings: WeatherStationReportViewSettings,
    pub crr_view_settings: CrrViewSettings,
    pub checklist_view_settings: ChecklistViewSettings,
    pub opaque_group_display_precedences: Vec<DisplayPrecedence>,
    pub semi_transparent_group_display_precedences: Vec<DisplayPrecedence>,
    pub show_crr_fix: bool,
    pub requested_altimeters: Vec<String>,
    pub requested_weather_reports: Vec<String>,
    pub altitude_limits_targets: AltitudeFilter,
    pub altitude_limits_ldbs: AltitudeFilter,
    pub visible_markers: Vec<String>,
    pub visible_centerlines: Vec<AirportRunway>,
    pub tearoffs: Vec<Tearoff>,
    pub master_toolbar_visible: bool,
    pub active_geo_map: String,
    pub map_filters: String,
    pub radar_filters: String,
    pub db_field_filters: String,
    pub history_count: i32,
    pub leader_length: i32,
    pub rdb_offset: String,
    pub id: String,
    pub facility_id: String,
    pub invert_numeric_keypad: Option<bool>,
    pub disable_mouse_pan_zoom: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$type")]
#[allow(clippy::large_enum_variant)]
pub enum DisplaySettings {
    #[serde(rename = "Vatsim.Nas.Crc.Ui.Displays.Asdex.Settings.AsdexDisplaySettings, CRC")]
    Asdex(AsdexDisplaySettings),
    #[serde(rename = "Vatsim.Nas.Crc.Ui.Displays.Stars.Settings.StarsDisplaySettings, CRC")]
    Stars(StarsDisplaySettings),
    #[serde(rename = "Vatsim.Nas.Crc.Ui.Displays.TowerCab.Settings.TowerCabDisplaySettings, CRC")]
    TowerCab(TowerCabDisplaySettings),
    #[serde(rename = "Vatsim.Nas.Crc.Ui.Displays.Eram.Settings.EramDisplaySettings, CRC")]
    Eram(EramDisplaySettings),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DisplayBookmark {
    pub index: i32,
    pub center: Point,
    pub range: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_down_mode_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct AsdexPrefSet {
    pub id: Option<String>,
    pub title: Option<String>,
    pub windows: Vec<Window>,
    pub show_coast_list: bool,
    pub coast_list_location: Margins,
    pub dcb_position: String,
    pub show_altitude_in_db: bool,
    pub show_aircraft_type_in_db: bool,
    pub show_sensors_in_db: bool,
    pub show_aircraft_category_in_db: bool,
    pub show_fix_in_db: bool,
    pub show_velocity_in_db: bool,
    pub show_scratchpads_in_db: bool,
    pub alert_message_location: Margins,
    pub preview_area_location: Margins,
    pub vector_length: i32,
    pub lists_brightness: i32,
    pub dcb_brightness: i32,
    pub dcb_char_size: i32,
    pub coast_suspend_char_size: i32,
    pub preview_area_char_size: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct StarsPrefSet {
    pub id: Option<String>,
    pub version: i32,
    pub name: Option<String>,
    pub range: f64,
    pub display_center: Option<Point>,
    pub display_off_center: bool,
    pub range_ring_spacing: i32,
    pub range_ring_center: Option<Point>,
    pub range_rings_off_center: bool,
    pub leader_dir_tracked: String,
    pub leader_dir_associated: String,
    pub leader_dir_unassociated: String,
    pub leader_length: i32,
    pub history_count: i32,
    pub ptl_length: f64,
    pub ptl_own: bool,
    pub ptl_all: bool,
    pub visible_lists: String,
    pub selected_video_map_list_type: String,
    pub visible_ssa_fields: String,
    pub all_ssa_fields_visible: bool,
    pub visible_gi_text_lines: String,
    pub brightness_dcb: i32,
    pub brightness_mpa: i32,
    pub brightness_mpb: i32,
    pub brightness_fdb: i32,
    pub brightness_lst: i32,
    pub brightness_pos: i32,
    pub brightness_ldb: i32,
    pub brightness_oth: i32,
    pub brightness_tls: i32,
    pub brightness_rr: i32,
    pub brightness_cmp: i32,
    pub brightness_bcn: i32,
    pub brightness_pri: i32,
    pub brightness_hst: i32,
    pub char_size_data_blocks: i32,
    pub char_size_lists: i32,
    pub char_size_dcb: i32,
    pub char_size_tools: i32,
    pub char_size_position_symbols: i32,
    pub preview_area_location: String,
    pub ssa_location: String,
    pub tab_list_location: String,
    pub vfr_list_location: String,
    pub la_ca_mci_list_location: String,
    pub coast_suspend_list_location: String,
    pub sign_on_list_location: String,
    pub video_map_list_location: String,
    pub crda_list_location: String,
    #[serde(rename = "TowerList1Location")]
    pub tower_list1location: String,
    #[serde(rename = "TowerList2Location")]
    pub tower_list2location: String,
    #[serde(rename = "TowerList3Location")]
    pub tower_list3location: String,
    pub tab_list_size: i32,
    pub coast_suspend_list_size: i32,
    pub vfr_list_size: i32,
    #[serde(rename = "TowerList1Size")]
    pub tower_list1size: i32,
    #[serde(rename = "TowerList2Size")]
    pub tower_list2size: i32,
    #[serde(rename = "TowerList3Size")]
    pub tower_list3size: i32,
    pub selected_video_map_ids: Vec<i32>,
    pub altitude_filter_unassociated: AltitudeFilter,
    pub altitude_filter_associated: AltitudeFilter,
    pub quick_looked_tcps: Vec<Tcp>,
    pub quick_look_all: bool,
    pub selected_beacon_codes: Vec<String>,
    pub dcb_location: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct AltitudeFilter {
    pub low: i32,
    pub high: i32,
    pub is_valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct Window {
    pub id: String,
    pub display_type: String,
    pub bounds: Bounds,
    pub center: Point,
    pub range: f64,
    pub rotation: f64,
    pub enable_anti_aliasing: bool,
    pub background_brightness: i32,
    pub hold_bars_brightness: i32,
    pub movement_areas_brightness: i32,
    pub track_brightness: i32,
    pub data_blocks_brightness: i32,
    pub temp_map_areas_brightness: i32,
    pub temp_map_text_brightness: i32,
    pub data_block_trait_areas: Vec<DataBlockTraitArea>,
    pub show_data_blocks: bool,
    pub full_data_blocks: bool,
    pub data_block_char_size: i32,
    pub temp_data_char_size: i32,
    pub show_history: bool,
    pub history_length: i32,
    pub leader_direction: String,
    pub leader_length: i32,
    pub show_vector_line: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub location: Margins,
    pub size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DataBlockTraitArea {
    pub area: Area,
    pub traits: Traits,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Area {
    pub points: Vec<Point>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
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
    pub data_blocks_char_size: i32,
    pub data_blocks_brightness: i32,
    pub show_vector: bool,
    pub leader_length: i32,
    pub leader_direction: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_field_names)]
pub struct Margins {
    pub left_margin: Option<i32>,
    pub right_margin: Option<i32>,
    pub top_margin: Option<i32>,
    pub bottom_margin: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DisplayWindowBookmark {
    pub index: i32,
    pub selected_display_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct GlobalBookmark {
    pub index: i32,
    pub selected_display_window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct InformationWindowSettings {
    #[serde(rename = "Type")]
    pub type_field: String,
    pub window_settings: WindowSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Bcgs {
    pub background: i32,
    pub cursor: i32,
    pub text: i32,
    pub paired_target: i32,
    pub unpaired_target: i32,
    pub paired_history: i32,
    pub unpaired_history: i32,
    pub ldb: i32,
    pub weather: i32,
    pub nexrad: i32,
    pub system_brightness: i32,
    pub button: i32,
    pub border: i32,
    pub toolbar: i32,
    pub toolbar_border: i32,
    pub active_border_border: i32,
    pub fdb: i32,
    pub sat_comm: i32,
    pub on_freq: i32,
    pub fence: i32,
    pub db_fel: i32,
    pub outage: i32,
    pub non_adsb: i32,
    pub map_group1: i32,
    pub map_group2: i32,
    pub map_group3: i32,
    pub map_group4: i32,
    pub map_group5: i32,
    pub map_group6: i32,
    pub map_group7: i32,
    pub map_group8: i32,
    pub map_group9: i32,
    pub map_group10: i32,
    pub map_group11: i32,
    pub map_group12: i32,
    pub map_group13: i32,
    pub map_group14: i32,
    pub map_group15: i32,
    pub map_group16: i32,
    pub map_group17: i32,
    pub map_group18: i32,
    pub map_group19: i32,
    pub map_group20: i32,
    pub map_group21: i32,
    pub map_group22: i32,
    pub map_group23: i32,
    pub map_group24: i32,
    pub map_group25: i32,
    pub map_group26: i32,
    pub map_group27: i32,
    pub map_group28: i32,
    pub map_group29: i32,
    pub map_group30: i32,
    pub map_group31: i32,
    pub map_group32: i32,
    pub map_group33: i32,
    pub map_group34: i32,
    pub map_group35: i32,
    pub map_group36: i32,
    pub map_group37: i32,
    pub map_group38: i32,
    pub map_group39: i32,
    pub map_group40: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct BcgModifiers {
    pub sldb: i32,
    pub portal: i32,
    pub line4: i32,
    pub dwell: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct FontSizes {
    pub fdb: i32,
    pub toolbar: i32,
    pub rdb: i32,
    pub ldb: i32,
    pub outage: i32,
    pub line4: i32,
    pub portal: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct TimeViewSettings {
    pub location: Location,
    pub show_border: bool,
    pub brightness: i32,
    pub font_size: i32,
    pub is_opaque: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub location: String,
    pub anchor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct McaViewSettings {
    pub location: Location,
    pub preview_area_lines: i32,
    pub width: i32,
    pub font_size: i32,
    pub brightness: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ResponseAreaViewSettings {
    pub location: Location,
    pub width: i32,
    pub font_size: i32,
    pub brightness: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct AltimeterSettingsViewSettings {
    pub show_tearoffs: bool,
    pub lines: i32,
    pub columns: i32,
    pub font_size: i32,
    pub brightness: i32,
    pub manually_sort: bool,
    pub location: Location,
    pub is_opaque: bool,
    pub show_border: bool,
    pub is_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct BeaconCodeViewSettings {
    pub lines: i32,
    pub columns: i32,
    pub font_size: i32,
    pub brightness: i32,
    pub manually_sort: bool,
    pub location: Location,
    pub is_opaque: bool,
    pub show_border: bool,
    pub is_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct WeatherStationReportViewSettings {
    pub show_tearoffs: bool,
    pub lines: i32,
    pub font_size: i32,
    pub brightness: i32,
    pub location: Location,
    pub is_opaque: bool,
    pub show_border: bool,
    pub is_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[allow(clippy::struct_excessive_bools)]
pub struct CrrViewSettings {
    pub lines: i32,
    pub font_size: i32,
    pub brightness: i32,
    pub view_list: bool,
    pub selected_color: String,
    pub color_brightness: ColorBrightness,
    pub location: Location,
    pub is_opaque: bool,
    pub show_border: bool,
    pub is_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ColorBrightness {
    pub white: i32,
    pub coral: i32,
    pub green: i32,
    pub yellow: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct ChecklistViewSettings {
    pub lines: i32,
    pub font_size: i32,
    pub highlight_brightness: i32,
    pub brightness: i32,
    pub location: Location,
    pub is_opaque: bool,
    pub show_border: bool,
    pub is_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct DisplayPrecedence {
    pub precedence: String,
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Tearoff {
    #[serde(rename = "$type")]
    pub type_field: String,
    #[serde(rename = "Type")]
    pub type_field2: String,
    pub id: String,
    pub location: Location,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Tcp {
    pub subset: i32,
    pub sector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AirportRunway {
    pub airport_id: String,
    pub runway_id: String,
}
