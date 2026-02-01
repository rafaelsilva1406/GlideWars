variable "resource_group_name" {
  description = "Name of the Azure resource group"
  type        = string
  default     = "rg-glidewars"
}

variable "location" {
  description = "Azure region for resources"
  type        = string
  default     = "eastus"
}

variable "environment" {
  description = "Environment name (dev, staging, prod)"
  type        = string
  default     = "prod"
}

variable "project_name" {
  description = "Project name used for resource naming"
  type        = string
  default     = "glidewars"
}

variable "acr_name" {
  description = "Azure Container Registry name (must be globally unique)"
  type        = string
  default     = "acrglidewars"
}

variable "dns_name_label" {
  description = "DNS name label for the container instance (must be globally unique)"
  type        = string
  default     = "glidewars-game"
}

variable "image_name" {
  description = "Docker image name"
  type        = string
  default     = "glidewars"
}

variable "image_tag" {
  description = "Docker image tag"
  type        = string
  default     = "latest"
}

# Optional: SQL Database variables
# variable "sql_server_name" {
#   description = "Azure SQL Server name (must be globally unique)"
#   type        = string
#   default     = "sql-glidewars"
# }
#
# variable "sql_admin_username" {
#   description = "SQL Server administrator username"
#   type        = string
#   sensitive   = true
# }
#
# variable "sql_admin_password" {
#   description = "SQL Server administrator password"
#   type        = string
#   sensitive   = true
# }
