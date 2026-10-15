<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post-pet-serverError</name>
   <tag></tag>
   <elementGuidId>a6384a1b-9df5-41b1-a422-890e147ce069</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${petIdNotFound},\n  \&quot;category\&quot;: [\&quot;invalid_type_array\&quot;],\n  \&quot;name\&quot;: \&quot;doggie\&quot;,\n  \&quot;photoUrls\&quot;: [\&quot;string\&quot;],\n  \&quot;tags\&quot;: [],\n  \&quot;status\&quot;: \&quot;available\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e9cbadce-80f0-41d5-9abe-c17ea4022f87</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>50c3d807-1a29-48cd-bd6d-d4b4001d99fc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/v2/pet</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>f1eacf05-3c6b-4921-8f89-99cbe1edc1d9</id>
      <name>SchemaValidation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Data/postSchemaNotFound.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>0</defaultValue>
      <description></description>
      <id>0103b12f-7b95-439a-bb74-3d46490e1503</id>
      <masked>false</masked>
      <name>petIdNotFound</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()








WS.verifyResponseStatusCode(response, 500)

assertThat(response.getStatusCode()).isEqualTo(500)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
