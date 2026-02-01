# Glide Wars - Azure Deployment with Terraform

This directory contains Terraform configuration for deploying Glide Wars to Azure.

## Prerequisites

1. **Azure CLI**: Install from https://docs.microsoft.com/en-us/cli/azure/install-azure-cli
2. **Terraform**: Install from https://www.terraform.io/downloads
3. **Docker**: For building and pushing images

## Setup

### 1. Azure Authentication

```bash
# Login to Azure
az login

# Set your subscription (if you have multiple)
az account set --subscription "YOUR_SUBSCRIPTION_ID"
```

### 2. Configure Variables

```bash
# Copy the example variables file
cp terraform.tfvars.example terraform.tfvars

# Edit terraform.tfvars with your values
# Make sure acr_name and dns_name_label are globally unique
```

### 3. Initialize Terraform

```bash
cd terraform
terraform init
```

## Deployment Steps

### Step 1: Plan the Deployment

```bash
terraform plan
```

Review the planned changes to ensure everything looks correct.

### Step 2: Apply the Configuration

```bash
terraform apply
```

Type `yes` when prompted to confirm the deployment.

### Step 3: Build and Push Docker Image

After Terraform creates the resources, build and push your Docker image:

```bash
# Get ACR credentials
ACR_NAME=$(terraform output -raw container_registry_login_server | cut -d'.' -f1)
ACR_USERNAME=$(terraform output -raw container_registry_admin_username)
ACR_PASSWORD=$(az acr credential show --name $ACR_NAME --query "passwords[0].value" -o tsv)

# Login to ACR
echo $ACR_PASSWORD | docker login $(terraform output -raw container_registry_login_server) \
  --username $ACR_USERNAME --password-stdin

# Build the image
cd ..
docker build -t glidewars:latest .

# Tag and push
ACR_SERVER=$(terraform output -raw container_registry_login_server)
docker tag glidewars:latest $ACR_SERVER/glidewars:latest
docker push $ACR_SERVER/glidewars:latest
```

### Step 4: Restart Container Instance

```bash
# Restart the container to use the new image
az container restart \
  --name glidewars-frontend \
  --resource-group $(terraform output -raw resource_group_name)
```

### Step 5: Access Your Game

```bash
# Get the game URL
terraform output game_url
```

Visit the URL in your browser to play Glide Wars!

## Managing the Deployment

### Update the Deployment

Make changes to your Terraform files and run:

```bash
terraform plan
terraform apply
```

### Destroy the Deployment

To tear down all resources:

```bash
terraform destroy
```

Type `yes` when prompted.

## Costs

This configuration uses:
- Azure Container Registry (Basic): ~$5/month
- Azure Container Instances (0.5 CPU, 1GB RAM): ~$13/month
- **Total: ~$18/month**

Optional resources (commented out):
- Azure SQL Database (Basic): ~$5/month
- Azure CDN: ~$0.081/GB + $0.0075/10K requests

## Customization

### Scaling

To scale the container instance, edit `main.tf`:

```hcl
container {
  cpu    = "1.0"    # Increase CPU
  memory = "2.0"    # Increase memory
}
```

### Enable Database

Uncomment the SQL Server resources in `main.tf` and add the SQL variables to your `terraform.tfvars`:

```hcl
sql_admin_username = "sqladmin"
sql_admin_password = "YourSecurePassword123!"
```

### Enable CDN

Uncomment the CDN resources in `main.tf` for global content delivery.

## Troubleshooting

### Container won't start

Check container logs:

```bash
az container logs \
  --name glidewars-frontend \
  --resource-group $(terraform output -raw resource_group_name)
```

### Access denied to ACR

Ensure admin user is enabled:

```bash
az acr update --name YOUR_ACR_NAME --admin-enabled true
```

### DNS name already taken

Change the `dns_name_label` in `terraform.tfvars` to something unique.

## Security Notes

- Never commit `terraform.tfvars` to version control
- Use Azure Key Vault for sensitive values in production
- Enable TLS/SSL for production deployments
- Configure network security groups if needed
