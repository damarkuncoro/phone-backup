# phone-backup-calendar 📅

Specialist crate for Android Calendar and iCalendar (.ics / RFC 5545) event extraction, recurrence rule calculation, and timetable indexing.

## 🏗 Architecture & Modules

- **`domain/`**: Calendar event models (`CalendarEvent`, `RecurrenceRule`, start/end times, reminders, attendees, geo-location).
- **`parsers/`**:
  - `android_parser.rs`: Live Android content provider query parser (`content://com.android.calendar/events`).
  - `ics_parser.rs`: RFC 5545 iCalendar stream and file parser.
- **`exporters/`**: RFC 5545 standard `.ics` exporter, JSON timetable compiler, and Markdown agenda generator.

## 🚀 Key Features

- **Direct Hardware ADB Extraction**: Queries Android Calendar Provider in real-time over ADB without requiring root privileges.
- **Recurrence Engine**: Full parsing and computation of recurrence rules (`RRULE:FREQ=DAILY|WEEKLY|MONTHLY|YEARLY`).
- **Standard Cross-Platform Compatibility**: Output `.ics` files import natively into Google Calendar, Apple Calendar, and Microsoft Outlook.
