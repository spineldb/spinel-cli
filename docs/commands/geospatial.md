# Geospatial Commands

Geospatial commands are used to store and query spatial data (longitude, latitude).

| Command | Usage | Summary |
| --- | --- | --- |
| `GEOADD` | `GEOADD key longitude latitude member [longitude latitude member ...]` | Adds one or more geospatial items (longitude, latitude, name) to the specified key. |
| `GEODIST` | `GEODIST key member1 member2 [unit]` | Returns the distance between two members in the geospatial index. |
| `GEOHASH` | `GEOHASH key member [member ...]` | Returns valid Geohash strings representing the position of one or more members. |
| `GEOPOS` | `GEOPOS key member [member ...]` | Returns the longitude and latitude of one or more members. |
| `GEORADIUS` | `GEORADIUS key longitude latitude radius m|km|ft|mi [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count]` | Queries a geospatial index for members within a given radius. |
| `GEORADIUSBYMEMBER` | `GEORADIUSBYMEMBER key member radius m|km|ft|mi [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT count]` | Queries a geospatial index for members within a given radius of a member. |
