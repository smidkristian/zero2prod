## Deploy

### Google Cloud

#### Creating a new project

Login and set project in the current shell session:

`gcloud auth login`
`gcloud config set project <project-id>`

Create a new repository:
`gcloud artifacts repositories create <repository-name> --repository-format=docker --location=europe-west3`

// europe-west3 is a location for Frankfurt

You may need to configure docker credentials:
`gcloud auth configure-docker europe-west3-docker.pkg.dev`

Tag docker image:
`docker tag <image-name> europe-west3-docker.pkg.dev/<project-id>/<repository-name>/<image-name>`

Push docker image:
`docker push europe-west3-docker.pkg.dev/<project-id>/<repository-name>/<image-name>`
