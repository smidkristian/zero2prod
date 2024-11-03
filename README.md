## Deploy

### Google Cloud

#### Creating a new project

Login and set project in the current shell session:

`gcloud auth login`

`gcloud config set project <project-id>`

Create a new repository:
`gcloud artifacts repositories create <repository-name> --repository-format=docker --location=europe-west3`

**Note:** _europe-west3_ is a location for Frankfurt
