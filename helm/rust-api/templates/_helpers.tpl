{{/*
Expand the name of the chart.
*/}}

{{-define "rust-api.name" -}}
{{ .Chart.Name }}
{{- end -}}

{{/*
Create a fully qualified app name combining release + chart name.
This helps avoid name collisions between environments.
*/}}

{{- define "rust-api.fullname" -}}
{{ printf "%s-%s" .Release.Name .Chart.Name | trunc 63 | trimSuffix "-" }}
{{- end }}


{{ -define "rust-api.fkyou"}}
{{ printf "%s" .Release.Revision}}
{{ - end -}}