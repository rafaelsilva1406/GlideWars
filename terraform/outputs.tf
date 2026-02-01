output "resource_group_name" {
  description = "The name of the resource group"
  value       = azurerm_resource_group.glidewars.name
}

output "resource_group_location" {
  description = "The location of the resource group"
  value       = azurerm_resource_group.glidewars.location
}

output "container_registry_login_server" {
  description = "The login server URL for the container registry"
  value       = azurerm_container_registry.glidewars.login_server
}

output "container_registry_admin_username" {
  description = "The admin username for the container registry"
  value       = azurerm_container_registry.glidewars.admin_username
  sensitive   = true
}

output "game_url" {
  description = "The URL where the game is accessible"
  value       = "http://${azurerm_container_group.glidewars_frontend.fqdn}"
}

output "game_fqdn" {
  description = "The FQDN of the container instance"
  value       = azurerm_container_group.glidewars_frontend.fqdn
}

output "game_ip_address" {
  description = "The public IP address of the game frontend"
  value       = azurerm_container_group.glidewars_frontend.ip_address
}

# Optional: Database outputs
# output "sql_server_fqdn" {
#   description = "The FQDN of the SQL Server"
#   value       = azurerm_mssql_server.glidewars.fully_qualified_domain_name
# }
#
# output "database_name" {
#   description = "The name of the database"
#   value       = azurerm_mssql_database.glidewars.name
# }

# Optional: CDN outputs
# output "cdn_endpoint_url" {
#   description = "The URL of the CDN endpoint"
#   value       = "https://${azurerm_cdn_endpoint.glidewars.fqdn}"
# }
