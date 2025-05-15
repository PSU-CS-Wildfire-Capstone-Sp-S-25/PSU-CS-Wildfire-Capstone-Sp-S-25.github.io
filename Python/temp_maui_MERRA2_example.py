import xarray as xr
import matplotlib.pyplot as plt
import cartopy.crs as ccrs
import cartopy.feature as cfeature
import numpy as np
from datetime import datetime

# --- Configuration ---
DATA_PATH = "daymet_v4_daily_hi_tmax_2023.nc"
VARIABLE = "tmax"
MAUI_BOUNDS = {
    "lat_min": 20.5,
    "lat_max": 21.1,
    "lon_min": -156.8,
    "lon_max": -156.2
}

# --- Load Dataset ---
ds = xr.open_dataset(DATA_PATH)
temperature = ds[VARIABLE]
lat = ds['lat']
lon = ds['lon']

# --- Create Mask for Maui Region ---
maui_mask = (
    (lat >= MAUI_BOUNDS["lat_min"]) & (lat <= MAUI_BOUNDS["lat_max"]) &
    (lon >= MAUI_BOUNDS["lon_min"]) & (lon <= MAUI_BOUNDS["lon_max"])
)

# Apply mask: NaNs outside Maui
temperature_maui = temperature.where(maui_mask)

# --- Time Loop ---
current_time = np.datetime64(datetime(2023, 8, 7, 12))
end_time = np.datetime64(datetime(2023, 8, 17, 12))

while current_time <= end_time:
    temp_day = temperature_maui.sel(time=current_time)

    plt.figure(figsize=(8, 6))
    ax = plt.axes(projection=ccrs.PlateCarree())

    # --- Set extent to Maui region ---
    ax.set_extent([
        MAUI_BOUNDS["lon_min"], MAUI_BOUNDS["lon_max"],
        MAUI_BOUNDS["lat_min"], MAUI_BOUNDS["lat_max"]
    ])

    # --- Add map features ---
    ax.add_feature(cfeature.COASTLINE)
    ax.add_feature(cfeature.LAND)
    ax.add_feature(cfeature.OCEAN)
    ax.add_feature(cfeature.BORDERS, linestyle=':')
    ax.add_feature(cfeature.STATES, linestyle=':')

    # --- Plot temperature ---
    p = ax.pcolormesh(
        lon, lat, temp_day,
        transform=ccrs.PlateCarree(),
        cmap='coolwarm',
        shading='auto', 
        vmax=32, vmin=20
    )

    plt.colorbar(p, ax=ax, label="Temperature (°C)")
    plt.title(f"Daily Max Temperature - {str(current_time)[:10]}")
    plt.tight_layout()
    plt.savefig(f"data/temp_map_{str(current_time)[:10]}.png")
    plt.close()

    current_time += np.timedelta64(1, 'D')

print("Daily temperature maps generated.")
