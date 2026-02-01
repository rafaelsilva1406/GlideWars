terraform {
  required_version = ">= 1.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 3.0"
    }
  }
}

provider "azurerm" {
  features {}
}

# Resource Group
resource "azurerm_resource_group" "glidewars" {
  name     = var.resource_group_name
  location = var.location

  tags = {
    Environment = var.environment
    Project     = "GlideWars"
    ManagedBy   = "Terraform"
  }
}

# Container Registry
resource "azurerm_container_registry" "glidewars" {
  name                = var.acr_name
  resource_group_name = azurerm_resource_group.glidewars.name
  location            = azurerm_resource_group.glidewars.location
  sku                 = "Basic"
  admin_enabled       = true

  tags = {
    Environment = var.environment
    Project     = "GlideWars"
  }
}

# Container Instance for Game Frontend
resource "azurerm_container_group" "glidewars_frontend" {
  name                = "${var.project_name}-frontend"
  location            = azurerm_resource_group.glidewars.location
  resource_group_name = azurerm_resource_group.glidewars.name
  os_type             = "Linux"
  dns_name_label      = var.dns_name_label
  restart_policy      = "Always"

  container {
    name   = "glidewars-game"
    image  = "${azurerm_container_registry.glidewars.login_server}/${var.image_name}:${var.image_tag}"
    cpu    = "0.5"
    memory = "1.0"

    ports {
      port     = 80
      protocol = "TCP"
    }

    environment_variables = {
      ENVIRONMENT = var.environment
    }
  }

  image_registry_credential {
    server   = azurerm_container_registry.glidewars.login_server
    username = azurerm_container_registry.glidewars.admin_username
    password = azurerm_container_registry.glidewars.admin_password
  }

  tags = {
    Environment = var.environment
    Project     = "GlideWars"
  }
}

# Optional: Azure SQL Database for leaderboards
# resource "azurerm_mssql_server" "glidewars" {
#   name                         = var.sql_server_name
#   resource_group_name          = azurerm_resource_group.glidewars.name
#   location                     = azurerm_resource_group.glidewars.location
#   version                      = "12.0"
#   administrator_login          = var.sql_admin_username
#   administrator_login_password = var.sql_admin_password
#   minimum_tls_version          = "1.2"
#
#   tags = {
#     Environment = var.environment
#     Project     = "GlideWars"
#   }
# }
#
# resource "azurerm_mssql_database" "glidewars" {
#   name           = "glidewars"
#   server_id      = azurerm_mssql_server.glidewars.id
#   collation      = "SQL_Latin1_General_CP1_CI_AS"
#   license_type   = "LicenseIncluded"
#   max_size_gb    = 2
#   sku_name       = "Basic"
#
#   tags = {
#     Environment = var.environment
#     Project     = "GlideWars"
#   }
# }

# Optional: CDN for static assets
# resource "azurerm_cdn_profile" "glidewars" {
#   name                = "${var.project_name}-cdn"
#   location            = azurerm_resource_group.glidewars.location
#   resource_group_name = azurerm_resource_group.glidewars.name
#   sku                 = "Standard_Microsoft"
#
#   tags = {
#     Environment = var.environment
#     Project     = "GlideWars"
#   }
# }
#
# resource "azurerm_cdn_endpoint" "glidewars" {
#   name                = "${var.project_name}-endpoint"
#   profile_name        = azurerm_cdn_profile.glidewars.name
#   location            = azurerm_resource_group.glidewars.location
#   resource_group_name = azurerm_resource_group.glidewars.name
#
#   origin {
#     name      = "glidewars-origin"
#     host_name = azurerm_container_group.glidewars_frontend.fqdn
#   }
#
#   origin_host_header = azurerm_container_group.glidewars_frontend.fqdn
#
#   tags = {
#     Environment = var.environment
#     Project     = "GlideWars"
#   }
# }
