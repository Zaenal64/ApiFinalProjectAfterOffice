import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

//define header
def accept = 'application/json'

//define method
def userRequest = WS.sendRequest(findTestObject('Object Repository/petstore.swagger/Pet/Delete/delete-pet-deletePet', [('petId') : GlobalVariable.petId]))

//define request body
def requestBody = userRequest.getResponseBodyContent()
def jsonRequest = new JsonSlurper().parseText(requestBody)

def header = {
	accept 'application/json'
}

// assertion request
assert jsonRequest.code == 200
assert jsonRequest.type == "unknown"


//verification response
WS.verifyResponseStatusCode(userRequest, userRequest.getStatusCode())
WS.verifyElementPropertyValue(userRequest, "type", "unknown")

println("PetId: " + GlobalVariable.petId + " Has been successfully deleted")
