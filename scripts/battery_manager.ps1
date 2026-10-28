<#
.SYNOPSIS
    MAPai Laptop Battery Manager & Diagnostic Guardian.
.DESCRIPTION
    Provides real-time battery status, health analysis, cycle counts, low-battery alert daemon,
    and power report generation for Windows laptops.
.PARAMETER Action
    Status (default), Monitor, Report, or Saver.
#>
param(
    [ValidateSet("Status", "Monitor", "Report", "Saver")]
    [string]$Action = "Status",

    [int]$LowThreshold = 20,
    [int]$HighThreshold = 80,
    [int]$CheckIntervalSeconds = 60
)

function Get-BatteryHealth {
    $tempReport = Join-Path $env:TEMP "mapai_battery_report.html"
    & powercfg /batteryreport /output $tempReport | Out-Null

    $designCap = 0
    $fullCap = 0
    $cycles = 0
    $chem = "Unknown"
    $mfg = "Unknown"
    $model = "Unknown"

    if (Test-Path $tempReport) {
        $content = Get-Content $tempReport -Raw
        if ($content -match 'DESIGN CAPACITY<\/span><\/td><td>([\d,]+)\s*mWh') {
            $designCap = [double]($matches[1] -replace ',', '')
        }
        if ($content -match 'FULL CHARGE CAPACITY<\/span><\/td><td>([\d,]+)\s*mWh') {
            $fullCap = [double]($matches[1] -replace ',', '')
        }
        if ($content -match 'CYCLE COUNT<\/span><\/td><td>(\d+)') {
            $cycles = [int]$matches[1]
        }
        if ($content -match 'CHEMISTRY<\/span><\/td><td>([^<]+)') {
            $chem = $matches[1].Trim()
        }
        if ($content -match 'MANUFACTURER<\/span><\/td><td>([^<]+)') {
            $mfg = $matches[1].Trim()
        }
        if ($content -match 'NAME<\/span><\/td><td>([^<]+)') {
            $model = $matches[1].Trim()
        }
    }

    $healthPercent = if ($designCap -gt 0) { [math]::Round(($fullCap / $designCap) * 100, 2) } else { 0 }
    $wearPercent = if ($designCap -gt 0) { [math]::Round((1 - ($fullCap / $designCap)) * 100, 2) } else { 0 }

    return [PSCustomObject]@{
        Model = $model
        Manufacturer = $mfg
        Chemistry = $chem
        DesignCapacity_mWh = $designCap
        FullChargeCapacity_mWh = $fullCap
        HealthPercent = $healthPercent
        WearPercent = $wearPercent
        CycleCount = $cycles
        ReportPath = $tempReport
    }
}

function Show-BatteryStatus {
    Add-Type -AssemblyName System.Windows.Forms
    $p = [System.Windows.Forms.SystemInformation]::PowerStatus
    $pct = [math]::Round($p.BatteryLifePercent * 100, 1)
    $source = if ($p.PowerLineStatus -eq 'Online') { "AC Adapter (Charging / Plugged In)" } else { "Battery (Discharging)" }
    $remTime = if ($p.BatteryLifeRemaining -gt 0) {
        $ts = [TimeSpan]::FromSeconds($p.BatteryLifeRemaining)
        "$([int]$ts.TotalHours)h $($ts.Minutes)m"
    } else {
        if ($p.PowerLineStatus -eq 'Online') { "Connected to Power" } else { "Calculating..." }
    }

    $health = Get-BatteryHealth

    $sourceColor = if ($p.PowerLineStatus -eq 'Online') { "Green" } else { "Magenta" }
    $chargeColor = if ($pct -le 20) { "Red" } elseif ($pct -le 40) { "Yellow" } else { "Green" }
    $healthColor = if ($health.HealthPercent -ge 85) { "Green" } else { "Yellow" }

    Write-Host ""
    Write-Host "============================================================" -ForegroundColor Cyan
    Write-Host "        [MAPai] Laptop Battery Diagnostics & Health         " -ForegroundColor Yellow
    Write-Host "============================================================" -ForegroundColor Cyan
    Write-Host "  Power Source        : $source" -ForegroundColor $sourceColor
    Write-Host "  Current Charge      : $pct %" -ForegroundColor $chargeColor
    Write-Host "  Estimated Runtime   : $remTime" -ForegroundColor White
    Write-Host "------------------------------------------------------------" -ForegroundColor DarkGray
    Write-Host "  Battery Hardware    : $($health.Model) ($($health.Manufacturer)) - $($health.Chemistry)" -ForegroundColor White
    Write-Host "  Design Capacity     : $($health.DesignCapacity_mWh) mWh" -ForegroundColor White
    Write-Host "  Full Charge Capacity: $($health.FullChargeCapacity_mWh) mWh" -ForegroundColor White
    Write-Host "  Battery Health      : $($health.HealthPercent) % (Wear: $($health.WearPercent) %)" -ForegroundColor $healthColor
    Write-Host "  Cycle Count         : $($health.CycleCount) cycles" -ForegroundColor White
    Write-Host "============================================================" -ForegroundColor Cyan

    if ($pct -le 25 -and $p.PowerLineStatus -ne 'Online') {
        Write-Host "  [WARNING] Battery is critically low ($pct %)! Please plug in charger." -ForegroundColor Red
    }
    Write-Host ""
}

function Start-BatteryMonitor {
    Add-Type -AssemblyName System.Windows.Forms
    Write-Host "Starting MAPai Battery Guardian Monitor (Checking every $CheckIntervalSeconds s)..." -ForegroundColor Cyan
    Write-Host "Press Ctrl+C to stop." -ForegroundColor DarkGray

    $lastNotifiedLow = $false
    $lastNotifiedHigh = $false

    while ($true) {
        $p = [System.Windows.Forms.SystemInformation]::PowerStatus
        $pct = [math]::Round($p.BatteryLifePercent * 100, 1)
        $isOnline = ($p.PowerLineStatus -eq 'Online')

        if (-not $isOnline -and $pct -le $LowThreshold -and -not $lastNotifiedLow) {
            [System.Windows.Forms.MessageBox]::Show(
                "Warning: Battery level is at $pct%! Please plug in your laptop charger.",
                "MAPai Battery Guardian - Low Battery",
                [System.Windows.Forms.MessageBoxButtons]::OK,
                [System.Windows.Forms.MessageBoxIcon]::Warning
            )
            $lastNotifiedLow = $true
        } elseif ($pct -gt $LowThreshold) {
            $lastNotifiedLow = $false
        }

        if ($isOnline -and $pct -ge $HighThreshold -and -not $lastNotifiedHigh) {
            [System.Windows.Forms.MessageBox]::Show(
                "Notice: Battery has reached $pct%. Unplug charger to preserve battery longevity if desired.",
                "MAPai Battery Guardian - Charge Limit",
                [System.Windows.Forms.MessageBoxButtons]::OK,
                [System.Windows.Forms.MessageBoxIcon]::Information
            )
            $lastNotifiedHigh = $true
        } elseif ($pct -lt $HighThreshold) {
            $lastNotifiedHigh = $false
        }

        Start-Sleep -Seconds $CheckIntervalSeconds
    }
}

if ($Action -eq "Status") {
    Show-BatteryStatus
} elseif ($Action -eq "Monitor") {
    Start-BatteryMonitor
} elseif ($Action -eq "Report") {
    $report = Get-BatteryHealth
    Write-Host "Opening full HTML battery report in browser: $($report.ReportPath)" -ForegroundColor Green
    Start-Process $report.ReportPath
} elseif ($Action -eq "Saver") {
    Write-Host "Opening Windows Battery Saver Settings..." -ForegroundColor Cyan
    Start-Process "ms-settings:batterysaver"
}
