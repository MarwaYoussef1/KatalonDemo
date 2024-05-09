import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
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

WebUI.callTestCase(findTestCase('Demo tests/Test7_callableTestLogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_Esports/span_Members Management'))

WebUI.click(findTestObject('Object Repository/Page_Esports/h4_Mahmoud Aboelsouddd'))

WebUI.click(findTestObject('Object Repository/Page_Esports/button_Manage public profile'))

WebUI.click(findTestObject('Object Repository/Page_Esports/mat-icon_add'))

WebUI.click(findTestObject('Object Repository/Page_Esports/div_Brawlhalla'))

WebUI.setText(findTestObject('Object Repository/Page_Esports/input_In-game Name_in_game_name'), gameName)

WebUI.click(findTestObject('Object Repository/Page_Esports/input_text'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Esports/div_Select Characters'))

WebUI.waitForElementClickable(findTestObject('Page_Esports/div_QUEEN NAI'), 2)

WebUI.enhancedClick(findTestObject('Page_Esports/div_QUEEN NAI'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Esports/div_Select Preferred Platform'))

WebUI.enhancedClick(findTestObject('Object Repository/Page_Esports/div_Xbox'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Esports/div_Select Current ranking'))

WebUI.enhancedClick(findTestObject('Object Repository/Page_Esports/div_Tin 5'))

WebUI.click(findTestObject('Object Repository/Page_Esports/div_Select Peak ranking_ng-input'))

WebUI.enhancedClick(findTestObject('Page_Esports/div_Tin 4'))

WebUI.click(findTestObject('Object Repository/Page_Esports/button_Save'))

WebUI.click(findTestObject('Object Repository/Page_Esports/div_Game added successfully'))

WebUI.delay(5)

WebUI.click(findTestObject('Object Repository/Page_Esports/i_Bronze_icon-dots-vertical'))

WebUI.click(findTestObject('Object Repository/Page_Esports/a_Delete'))

WebUI.click(findTestObject('Object Repository/Page_Esports/button_Yes, Delete'))

WebUI.click(findTestObject('Object Repository/Page_Esports/div_Game Deleted Successfully'))

WebUI.click(findTestObject('Object Repository/Page_Esports/div_SEF'))

WebUI.click(findTestObject('Object Repository/Page_Esports/a_Log out'))

WebUI.closeBrowser()

