<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>post-pet-addNewPet</name>
   <tag></tag>
   <elementGuidId>aa90caf5-c2fc-4478-a814-6a1f291b937b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: ${petId},\n  \&quot;category\&quot;: {\n    \&quot;id\&quot;: 1,\n    \&quot;name\&quot;: \&quot;${categoryName}\&quot;\n  },\n  \&quot;name\&quot;: \&quot;${petName}\&quot;,\n  \&quot;photoUrls\&quot;: [\n    \&quot;https://example.com/photo.jpg\&quot;\n  ],\n  \&quot;tags\&quot;: [\n    {\n      \&quot;id\&quot;: 1,\n      \&quot;name\&quot;: \&quot;tag-1\&quot;\n    }\n  ],\n  \&quot;status\&quot;: \&quot;${status}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0560fa39-1162-4c18-9402-cb625f817c36</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>508a7f9b-9701-4fe7-b5f9-837ae5fccaa4</webElementGuid>
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
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>80d70905-a47c-4336-93ee-ac64cbc4f9b5</id>
      <masked>false</masked>
      <name>petId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3ab2ce4f-ee35-4e69-a6b1-f39b16afe2c8</id>
      <masked>false</masked>
      <name>petName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9f834593-1358-4a91-beac-b6f1e37e579d</id>
      <masked>false</masked>
      <name>categoryName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9edba7f2-2042-4115-9b2b-127fcbe6a78d</id>
      <masked>false</masked>
      <name>status</name>
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



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
