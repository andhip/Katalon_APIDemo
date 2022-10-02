<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getBank</name>
   <tag></tag>
   <elementGuidId>70f7402d-ecdb-4f76-a1d1-a89b0dc16f98</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>6b14aa83-502c-4654-afc1-4b7c454ee94e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://www.w3.org/2003/05/soap-envelope&quot; xmlns:blz=&quot;http://thomas-bayer.com/blz/&quot;>
   &lt;soap:Header/>
   &lt;soap:Body>
      &lt;blz:getBank>
         &lt;blz:blz>gero et&lt;/blz:blz>
      &lt;/blz:getBank>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceEndpoint>http://www.thomas-bayer.com/axis2/services/BLZService</soapServiceEndpoint>
   <soapServiceFunction>getBank</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <wsdlAddress>http://www.thomas-bayer.com/axis2/services/BLZService?wsdl</wsdlAddress>
</WebServiceRequestEntity>
