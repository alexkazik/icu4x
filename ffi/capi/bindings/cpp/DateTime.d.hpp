#ifndef DateTime_D_HPP
#define DateTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CalendarError.d.hpp"
#include "IsoWeekday.d.hpp"
#include "WeekOf.d.hpp"

class Calendar;
class Date;
class IsoDateTime;
class Time;
class WeekCalculator;
struct WeekOf;
class CalendarError;
class IsoWeekday;


namespace capi {
    typedef struct DateTime DateTime;
}

class DateTime {
public:

  inline static diplomat::result<std::unique_ptr<DateTime>, CalendarError> create_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar);

  inline static diplomat::result<std::unique_ptr<DateTime>, CalendarError> create_from_codes_in_calendar(std::string_view era_code, int32_t year, std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const Calendar& calendar);

  inline static std::unique_ptr<DateTime> create_from_date_and_time(const Date& date, const Time& time);

  inline std::unique_ptr<Date> date() const;

  inline std::unique_ptr<Time> time() const;

  inline std::unique_ptr<IsoDateTime> to_iso() const;

  inline std::unique_ptr<DateTime> to_calendar(const Calendar& calendar) const;

  inline uint8_t hour() const;

  inline uint8_t minute() const;

  inline uint8_t second() const;

  inline uint32_t nanosecond() const;

  inline uint16_t day_of_year() const;

  inline uint32_t day_of_month() const;

  inline IsoWeekday day_of_week() const;

  inline uint32_t week_of_month(IsoWeekday first_weekday) const;

  inline WeekOf week_of_year(const WeekCalculator& calculator) const;

  inline uint32_t ordinal_month() const;

  inline std::string month_code() const;

  inline int32_t year_in_era() const;

  inline std::string era() const;

  inline uint8_t months_in_year() const;

  inline uint8_t days_in_month() const;

  inline uint16_t days_in_year() const;

  inline std::unique_ptr<Calendar> calendar() const;

  inline const capi::DateTime* AsFFI() const;
  inline capi::DateTime* AsFFI();
  inline static const DateTime* FromFFI(const capi::DateTime* ptr);
  inline static DateTime* FromFFI(capi::DateTime* ptr);
  inline static void operator delete(void* ptr);
private:
  DateTime() = delete;
  DateTime(const DateTime&) = delete;
  DateTime(DateTime&&) noexcept = delete;
  DateTime operator=(const DateTime&) = delete;
  DateTime operator=(DateTime&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DateTime_D_HPP