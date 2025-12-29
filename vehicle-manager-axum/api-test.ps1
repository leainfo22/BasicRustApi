Invoke-RestMethod -Uri http://localhost:6579/

Invoke-RestMethod -Uri http://localhost:6579/vehicle -Method Get

Invoke-RestMethod -Uri http://localhost:6579/vehicle -Method Post

$Params = @{
  Uri = 'http://localhost:6579/vehicle'
  Method = 'Post'
  Body = @{
      manufacturer = 'Susuki'
      model = 'Swift'
      year = 2021
  } | ConvertTo-Json
  ContentType = 'application/json'
}
Invoke-RestMethod @Params

$Params = @{
    Uri = 'http://localhost:6579/vehicle?manufacturer=Chevrolet&model=Silverado&year=2024&first_name=Leandro&last_name=YourDad'
    Method = 'Post'
}
Invoke-RestMethod @Params
