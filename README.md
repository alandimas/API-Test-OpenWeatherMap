# API-Test-OpenWeatherMap
Question No 4

Testing Step :
-	Open project API Test OpenWeatherMap
-	Open Test Case
-	Open and Run Test Case = Get 5 Day Weather Forecast
-	Open and Run Test Case = Get Current Air Pollution
- Then, To See Respon Body and JSON Schema
-	Open Object Repository
-	Open and Run 5-Day Weather Forecast (To get 5 Day use cnt : 40)
-	Open And Run Air Pollution



Scenario : Get 5 day weather forecast of Jakarta Selatan

https://api.openweathermap.org/data/2.5/forecast?lat=-6.300641&lon=106.814095&appid=ea8a2abb9c937ad008452ea077b02beb&units=imperial&cnt=40&lang=id

Respon Body and JSON Schemac : 200 OK

{
  "cod":"200",
  "message":0,
  "cnt":40,
  "list":[
    {
      "dt":1739340000,
      "main":{
        "temp":89.82,
        "feels_like":94.69,
        "temp_min":89.82,
        "temp_max":92.82,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1005,
        "humidity":51,
        "temp_kf":-1.67
      },
      "weather":[
        {
          "id":802,
          "main":"Clouds",
          "description":"awan tersebar",
          "icon":"03d"
        }],
      "clouds":{
        "all":41
      },
      "wind":{
        "speed":6.42,
        "deg":346,
        "gust":5.53
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-12 06:00:00"
    },
    {
      "dt":1739350800,
      "main":{
        "temp":89.87,
        "feels_like":95.27,
        "temp_min":89.87,
        "temp_max":90.66,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1003,
        "humidity":52,
        "temp_kf":-0.44
      },
      "weather":[
        {
          "id":803,
          "main":"Clouds",
          "description":"awan pecah",
          "icon":"04d"
        }],
      "clouds":{
        "all":73
      },
      "wind":{
        "speed":12.06,
        "deg":359,
        "gust":8.37
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-12 09:00:00"
    },
    {
      "dt":1739361600,
      "main":{
        "temp":85.91,
        "feels_like":91.29,
        "temp_min":85.91,
        "temp_max":85.91,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":61,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":99
      },
      "wind":{
        "speed":6.22,
        "deg":24,
        "gust":5.21
      },
      "visibility":10000,
      "pop":0.23,
      "rain":{
        "3h":0.24
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-12 12:00:00"
    },
    {
      "dt":1739372400,
      "main":{
        "temp":83.34,
        "feels_like":88.5,
        "temp_min":83.34,
        "temp_max":83.34,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":68,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":99
      },
      "wind":{
        "speed":4.99,
        "deg":93,
        "gust":5.44
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-12 15:00:00"
    },
    {
      "dt":1739383200,
      "main":{
        "temp":80.92,
        "feels_like":84.99,
        "temp_min":80.92,
        "temp_max":80.92,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":73,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":94
      },
      "wind":{
        "speed":5.03,
        "deg":154,
        "gust":6.53
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-12 18:00:00"
    },
    {
      "dt":1739394000,
      "main":{
        "temp":79.79,
        "feels_like":79.79,
        "temp_min":79.79,
        "temp_max":79.79,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":73,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":2.75,
        "deg":202,
        "gust":3.24
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-12 21:00:00"
    },
    {
      "dt":1739404800,
      "main":{
        "temp":80.29,
        "feels_like":83.57,
        "temp_min":80.29,
        "temp_max":80.29,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":71,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4,
        "deg":194,
        "gust":6.11
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-13 00:00:00"
    },
    {
      "dt":1739415600,
      "main":{
        "temp":89.31,
        "feels_like":93.76,
        "temp_min":89.31,
        "temp_max":89.31,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":51,
        "temp_kf":0
      },
      "weather":[
        {
          "id":803,
          "main":"Clouds",
          "description":"awan pecah",
          "icon":"04d"
        }],
      "clouds":{
        "all":79
      },
      "wind":{
        "speed":3.36,
        "deg":234,
        "gust":5.08
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-13 03:00:00"
    },
    {
      "dt":1739426400,
      "main":{
        "temp":94.1,
        "feels_like":98.92,
        "temp_min":94.1,
        "temp_max":94.1,
        "pressure":1007,
        "sea_level":1007,
        "grnd_level":1003,
        "humidity":43,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":89
      },
      "wind":{
        "speed":7.76,
        "deg":355,
        "gust":8.12
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-13 06:00:00"
    },
    {
      "dt":1739437200,
      "main":{
        "temp":87.19,
        "feels_like":93.04,
        "temp_min":87.19,
        "temp_max":87.19,
        "pressure":1007,
        "sea_level":1007,
        "grnd_level":1002,
        "humidity":59,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":10.33,
        "deg":6,
        "gust":9.28
      },
      "visibility":10000,
      "pop":0.05,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-13 09:00:00"
    },
    {
      "dt":1739448000,
      "main":{
        "temp":84.18,
        "feels_like":90.54,
        "temp_min":84.18,
        "temp_max":84.18,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1004,
        "humidity":69,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":9.28,
        "deg":66,
        "gust":9.37
      },
      "visibility":10000,
      "pop":0.22,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-13 12:00:00"
    },
    {
      "dt":1739458800,
      "main":{
        "temp":82.69,
        "feels_like":88.29,
        "temp_min":82.69,
        "temp_max":82.69,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":72,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":6.24,
        "deg":111,
        "gust":8.61
      },
      "visibility":10000,
      "pop":0.26,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-13 15:00:00"
    },
    {
      "dt":1739469600,
      "main":{
        "temp":81.16,
        "feels_like":85.82,
        "temp_min":81.16,
        "temp_max":81.16,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1005,
        "humidity":75,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.56,
        "deg":131,
        "gust":4.59
      },
      "visibility":10000,
      "pop":0.18,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-13 18:00:00"
    },
    {
      "dt":1739480400,
      "main":{
        "temp":80.42,
        "feels_like":84.52,
        "temp_min":80.42,
        "temp_max":80.42,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1004,
        "humidity":76,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":2.64,
        "deg":195,
        "gust":3.58
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-13 21:00:00"
    },
    {
      "dt":1739491200,
      "main":{
        "temp":80.91,
        "feels_like":85.14,
        "temp_min":80.91,
        "temp_max":80.91,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1005,
        "humidity":74,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":2.53,
        "deg":256,
        "gust":3.06
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-14 00:00:00"
    },
    {
      "dt":1739502000,
      "main":{
        "temp":87.28,
        "feels_like":92.8,
        "temp_min":87.28,
        "temp_max":87.28,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":58,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4.18,
        "deg":301,
        "gust":4.14
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-14 03:00:00"
    },
    {
      "dt":1739512800,
      "main":{
        "temp":93.06,
        "feels_like":99.05,
        "temp_min":93.06,
        "temp_max":93.06,
        "pressure":1007,
        "sea_level":1007,
        "grnd_level":1003,
        "humidity":47,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":8.66,
        "deg":296,
        "gust":7.58
      },
      "visibility":10000,
      "pop":0.32,
      "rain":{
        "3h":0.34
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-14 06:00:00"
    },
    {
      "dt":1739523600,
      "main":{
        "temp":88.2,
        "feels_like":95.58,
        "temp_min":88.2,
        "temp_max":88.2,
        "pressure":1006,
        "sea_level":1006,
        "grnd_level":1002,
        "humidity":60,
        "temp_kf":0
      },
      "weather":[
        {
          "id":501,
          "main":"Rain",
          "description":"hujan sedang",
          "icon":"10d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":9.19,
        "deg":280,
        "gust":11.07
      },
      "visibility":5978,
      "pop":1,
      "rain":{
        "3h":3.09
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-14 09:00:00"
    },
    {
      "dt":1739534400,
      "main":{
        "temp":82.4,
        "feels_like":88.18,
        "temp_min":82.4,
        "temp_max":82.4,
        "pressure":1008,
        "sea_level":1008,
        "grnd_level":1004,
        "humidity":74,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":8.75,
        "deg":267,
        "gust":14.61
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":2.7
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-14 12:00:00"
    },
    {
      "dt":1739545200,
      "main":{
        "temp":81.19,
        "feels_like":86.27,
        "temp_min":81.19,
        "temp_max":81.19,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1005,
        "humidity":77,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":6.91,
        "deg":246,
        "gust":12.57
      },
      "visibility":10000,
      "pop":0.94,
      "rain":{
        "3h":0.84
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-14 15:00:00"
    },
    {
      "dt":1739556000,
      "main":{
        "temp":80.24,
        "feels_like":84.6,
        "temp_min":80.24,
        "temp_max":80.24,
        "pressure":1008,
        "sea_level":1008,
        "grnd_level":1004,
        "humidity":79,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4.34,
        "deg":255,
        "gust":7.87
      },
      "visibility":10000,
      "pop":0.57,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-14 18:00:00"
    },
    {
      "dt":1739566800,
      "main":{
        "temp":79.11,
        "feels_like":79.11,
        "temp_min":79.11,
        "temp_max":79.11,
        "pressure":1008,
        "sea_level":1008,
        "grnd_level":1003,
        "humidity":81,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.69,
        "deg":247,
        "gust":7.29
      },
      "visibility":10000,
      "pop":0.55,
      "rain":{
        "3h":0.39
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-14 21:00:00"
    },
    {
      "dt":1739577600,
      "main":{
        "temp":79.7,
        "feels_like":79.7,
        "temp_min":79.7,
        "temp_max":79.7,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1005,
        "humidity":79,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4.47,
        "deg":258,
        "gust":7.67
      },
      "visibility":10000,
      "pop":0.4,
      "rain":{
        "3h":0.11
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-15 00:00:00"
    },
    {
      "dt":1739588400,
      "main":{
        "temp":86.5,
        "feels_like":92.07,
        "temp_min":86.5,
        "temp_max":86.5,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1006,
        "humidity":60,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":93
      },
      "wind":{
        "speed":6.78,
        "deg":299,
        "gust":8.28
      },
      "visibility":10000,
      "pop":0.21,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-15 03:00:00"
    },
    {
      "dt":1739599200,
      "main":{
        "temp":89.04,
        "feels_like":94.55,
        "temp_min":89.04,
        "temp_max":89.04,
        "pressure":1008,
        "sea_level":1008,
        "grnd_level":1004,
        "humidity":54,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10d"
        }],
      "clouds":{
        "all":87
      },
      "wind":{
        "speed":10.36,
        "deg":298,
        "gust":11.63
      },
      "visibility":10000,
      "pop":0.81,
      "rain":{
        "3h":0.61
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-15 06:00:00"
    },
    {
      "dt":1739610000,
      "main":{
        "temp":85.8,
        "feels_like":91.44,
        "temp_min":85.8,
        "temp_max":85.8,
        "pressure":1007,
        "sea_level":1007,
        "grnd_level":1003,
        "humidity":62,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10d"
        }],
      "clouds":{
        "all":81
      },
      "wind":{
        "speed":8.37,
        "deg":313,
        "gust":9.51
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":1.61
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-15 09:00:00"
    },
    {
      "dt":1739620800,
      "main":{
        "temp":80.53,
        "feels_like":85.37,
        "temp_min":80.53,
        "temp_max":80.53,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1005,
        "humidity":80,
        "temp_kf":0
      },
      "weather":[
        {
          "id":501,
          "main":"Rain",
          "description":"hujan sedang",
          "icon":"10n"
        }],
      "clouds":{
        "all":90
      },
      "wind":{
        "speed":5.12,
        "deg":325,
        "gust":8.19
      },
      "visibility":4607,
      "pop":1,
      "rain":{
        "3h":4.4
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-15 12:00:00"
    },
    {
      "dt":1739631600,
      "main":{
        "temp":80.17,
        "feels_like":84.88,
        "temp_min":80.17,
        "temp_max":80.17,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":82,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.15,
        "deg":287,
        "gust":5.19
      },
      "visibility":4580,
      "pop":1,
      "rain":{
        "3h":1.86
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-15 15:00:00"
    },
    {
      "dt":1739642400,
      "main":{
        "temp":77.9,
        "feels_like":79.38,
        "temp_min":77.9,
        "temp_max":77.9,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1005,
        "humidity":85,
        "temp_kf":0
      },
      "weather":[
        {
          "id":501,
          "main":"Rain",
          "description":"hujan sedang",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.65,
        "deg":313,
        "gust":5.75
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":3.46
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-15 18:00:00"
    },
    {
      "dt":1739653200,
      "main":{
        "temp":75.29,
        "feels_like":76.98,
        "temp_min":75.29,
        "temp_max":75.29,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1004,
        "humidity":95,
        "temp_kf":0
      },
      "weather":[
        {
          "id":501,
          "main":"Rain",
          "description":"hujan sedang",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.09,
        "deg":215,
        "gust":5.12
      },
      "visibility":5528,
      "pop":1,
      "rain":{
        "3h":7.24
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-15 21:00:00"
    },
    {
      "dt":1739664000,
      "main":{
        "temp":75.63,
        "feels_like":77.18,
        "temp_min":75.63,
        "temp_max":75.63,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":91,
        "temp_kf":0
      },
      "weather":[
        {
          "id":501,
          "main":"Rain",
          "description":"hujan sedang",
          "icon":"10d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4.9,
        "deg":186,
        "gust":7
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":3.43
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-16 00:00:00"
    },
    {
      "dt":1739674800,
      "main":{
        "temp":80.44,
        "feels_like":84.4,
        "temp_min":80.44,
        "temp_max":80.44,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":75,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.62,
        "deg":261,
        "gust":5.84
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-16 03:00:00"
    },
    {
      "dt":1739685600,
      "main":{
        "temp":85.86,
        "feels_like":91.54,
        "temp_min":85.86,
        "temp_max":85.86,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1004,
        "humidity":62,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":99
      },
      "wind":{
        "speed":5.26,
        "deg":337,
        "gust":8.01
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-16 06:00:00"
    },
    {
      "dt":1739696400,
      "main":{
        "temp":85.64,
        "feels_like":92.61,
        "temp_min":85.64,
        "temp_max":85.64,
        "pressure":1007,
        "sea_level":1007,
        "grnd_level":1003,
        "humidity":66,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":8.16,
        "deg":306,
        "gust":9.95
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":2.16
      },
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-16 09:00:00"
    },
    {
      "dt":1739707200,
      "main":{
        "temp":82.02,
        "feels_like":88.11,
        "temp_min":82.02,
        "temp_max":82.02,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1004,
        "humidity":77,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":5.41,
        "deg":329,
        "gust":8.3
      },
      "visibility":10000,
      "pop":1,
      "rain":{
        "3h":0.94
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-16 12:00:00"
    },
    {
      "dt":1739718000,
      "main":{
        "temp":79.5,
        "feels_like":79.5,
        "temp_min":79.5,
        "temp_max":79.5,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":82,
        "temp_kf":0
      },
      "weather":[
        {
          "id":500,
          "main":"Rain",
          "description":"hujan rintik-rintik",
          "icon":"10n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":4.12,
        "deg":303,
        "gust":8.03
      },
      "visibility":10000,
      "pop":0.97,
      "rain":{
        "3h":1.03
      },
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-16 15:00:00"
    },
    {
      "dt":1739728800,
      "main":{
        "temp":77.59,
        "feels_like":79,
        "temp_min":77.59,
        "temp_max":77.59,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1005,
        "humidity":84,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":5.32,
        "deg":253,
        "gust":11.54
      },
      "visibility":10000,
      "pop":0.9,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-16 18:00:00"
    },
    {
      "dt":1739739600,
      "main":{
        "temp":77.29,
        "feels_like":78.66,
        "temp_min":77.29,
        "temp_max":77.29,
        "pressure":1009,
        "sea_level":1009,
        "grnd_level":1005,
        "humidity":84,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04n"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3.22,
        "deg":256,
        "gust":6.42
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"n"
      },
      "dt_txt":"2025-02-16 21:00:00"
    },
    {
      "dt":1739750400,
      "main":{
        "temp":78.19,
        "feels_like":79.52,
        "temp_min":78.19,
        "temp_max":78.19,
        "pressure":1010,
        "sea_level":1010,
        "grnd_level":1006,
        "humidity":81,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":3,
        "deg":238,
        "gust":5.1
      },
      "visibility":10000,
      "pop":0,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-17 00:00:00"
    },
    {
      "dt":1739761200,
      "main":{
        "temp":81.43,
        "feels_like":85.77,
        "temp_min":81.43,
        "temp_max":81.43,
        "pressure":1011,
        "sea_level":1011,
        "grnd_level":1007,
        "humidity":72,
        "temp_kf":0
      },
      "weather":[
        {
          "id":804,
          "main":"Clouds",
          "description":"awan mendung",
          "icon":"04d"
        }],
      "clouds":{
        "all":100
      },
      "wind":{
        "speed":5.95,
        "deg":277,
        "gust":8.14
      },
      "visibility":10000,
      "pop":0.31,
      "sys":{
        "pod":"d"
      },
      "dt_txt":"2025-02-17 03:00:00"
    }],
  "city":{
    "id":1642941,
    "name":"Jagakarsa",
    "coord":{
      "lat":-6.3006,
      "lon":106.8141
    },
    "country":"ID",
    "population":0,
    "timezone":25200,
    "sunrise":1739314647,
    "sunset":1739359001
  }
}



Scenario : Get current air pollution of Jakarta Selatan

https://api.openweathermap.org/data/2.5/air_pollution?lat=-6.300641&lon=106.814095&appid=ea8a2abb9c937ad008452ea077b02beb

Respon Body and JSON Schemac : 200 OK

{
  "coord":{
    "lon":106.8141,
    "lat":-6.3006
  },
  "list":[
    {
      "main":{
        "aqi":5
      },
      "components":{
        "co":15594.48,
        "no":101.92,
        "no2":304.34,
        "o3":72.24,
        "so2":87.74,
        "pm2_5":459.79,
        "pm10":609.4,
        "nh3":46.61
      },
      "dt":1739330653
    }]
}

