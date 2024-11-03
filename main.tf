terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 4.0"
    }
  }
}

provider "google" {
  project = "zero2prod-473829"
  region  = "europe-west3"
}

# Cloud SQL PostgreSQL instance
resource "google_sql_database_instance" "zero2prod_db" {
  name             = "zero2prod-db"
  database_version = "POSTGRES_14"
  region          = "europe-west3"

  settings {
    tier              = "db-g1-small"
    availability_type = "ZONAL"
    
    backup_configuration {
      enabled                        = true
      start_time                     = "01:00"
      point_in_time_recovery_enabled = true
    }

    maintenance_window {
      day  = 7  # sunday
      hour = 1
    }

    disk_size       = 10
    disk_type       = "PD_SSD"
    disk_autoresize = false
  }

  root_password = data.google_secret_manager_secret_version.db_password.secret_data
}

# Cloud Run service
resource "google_cloud_run_service" "zero2prod_app" {
  name     = "zero2prod-app"
  location = "europe-west3"

  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/minScale" = "1"
        "autoscaling.knative.dev/maxScale" = "1"
      }
    }

    spec {
      containers {
        image = "europe-west3-docker.pkg.dev/zero2prod-473829/zero2prod/zero2prod:latest"
        
        ports {
          container_port = 8080
        }

        resources {
          limits = {
            cpu    = "1000m"
            memory = "256Mi"
          }
        }

        env {
          name  = "APP_ENVIRONMENT"
          value = "prod"
        }

        liveness_probe {
          http_get {
            path = "/health_check"
            port = 8080
          }
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

# Set public accessibility for "zero2prod_app"
resource "google_cloud_run_service_iam_policy" "noauth" {
  location = google_cloud_run_service.zero2prod_app.location
  project  = google_cloud_run_service.zero2prod_app.project
  service  = google_cloud_run_service.zero2prod_app.name

  policy_data = jsonencode({
    bindings = [{
      role = "roles/run.invoker"
      members = [
        "allUsers",
      ]
    }]
  })
}


# Get DB password from Secret Manager
data "google_secret_manager_secret_version" "db_password" {
  secret  = "DB_PASSWORD"
  version = "latest"
}
