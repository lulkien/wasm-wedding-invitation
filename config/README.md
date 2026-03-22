# Wedding Configuration

Edit `wedding.json` to customize ceremony details without changing code.

## Configuration Fields

- **ceremony** – Date and time
  - `date_utc` – ISO 8601 UTC (e.g. "2026-03-28T06:30:00Z" for 1:30 PM Vietnam UTC+7)
  - `date_display` – Human-readable date
  - `day_of_week`, `day_number`, `month_year` – For time section
  - `reception_time`, `ceremony_time` – Display times

- **venue** – Location
  - `name` – Venue name
  - `address` – Full address
  - `location_line`, `province` – For hero section
  - `maps_embed_url` – Google Maps embed iframe src
  - `maps_directions_url` – Link for "Go to Google Map" button

- **couple** – Names
  - `groom_name`, `bride_name` – Full names
  - `title` – Display title (e.g. "Hoang Kien & Pham Hang")

Changes require a rebuild (`dx build` or `cargo build`).
