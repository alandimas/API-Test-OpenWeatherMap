<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5-Day Weather Forecast</name>
   <tag></tag>
   <elementGuidId>e60600aa-dd08-446f-9e84-ce7010725dea</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.openweathermap.org/data/2.5/forecast?lat=-6.300641&amp;lon=106.814095&amp;appid=ea8a2abb9c937ad008452ea077b02beb&amp;units=imperial&amp;cnt=40&amp;lang=id</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getResponseText()).contains('Katalon Test Project')


def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())


String graphQLSchema = 
'''
type Author {
  id: ID!
  firstName: String!
  lastName: String!
}

type Query {
  findAllAuthors: [Author]!
  countAuthors: Long!
}

type Mutation {
  newAuthor(firstName: String!, lastName: String!) : Author!
}

type Query {
    findAllAuthors: [Author]!
    countAuthors: Long!
    findAllBooks: [Book]!
    countBooks: Long!
}

type Mutation {
    newAuthor(id: ID!, firstName: String!, lastName: String!) : Author!
    newBook(id: ID!, title: String!, isbn: String, pageCount: Int, author: ID!) : Book!
    deleteBook(id: ID!) : Boolean
    updateBookPageCount(pageCount: Int!, id: ID!) : Book!
}
'''

boolean successful = WS.validateGraphqlRequestAgainstSchema(request, graphQLSchema)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
