<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>put-pet-existing</name>
   <tag></tag>
   <elementGuidId>24c9a7e1-3e18-4ae2-b626-efb2a0535acd</elementGuidId>
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
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>3cb67349-af8f-4457-b52d-8f463aa67ed5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
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
      <id>7e799a2b-24c1-4882-aca7-77dc18d02453</id>
      <masked>false</masked>
      <name>petId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>151e9891-2625-47eb-9cd0-26a04761b2aa</id>
      <masked>false</masked>
      <name>petName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3e6b4377-c646-4f14-8977-41ec72bbe3d0</id>
      <masked>false</masked>
      <name>categoryName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>377f3980-02f8-4814-8a31-2c44f088bc52</id>
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
