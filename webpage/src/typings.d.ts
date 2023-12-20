declare namespace wakaTimeWebapp {
  export interface Data {
    year: number;
    total_time: number;
    time_by_language: Array<[string, number]>;
    time_by_project: Array<[string, number]>;
    time_by_month: Array<[number, number]>;
    time_by_month_day: Array<[number, number]>;
    time_by_weekday: Array<[string, number]>;
    time_by_day: Array<[number, number]>;
    time_by_hours: Array<[number, number]>;
    late_night_time: Array<[number, number]>;
  }
}
