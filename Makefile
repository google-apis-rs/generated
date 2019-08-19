MCP = ../target/release/mcp

help:
	$(info -- Target ----------------------------------------------------------------------------)
	$(info gen-all             | generate all libraries and CLIs)
	$(info --------------------------------------------------------------------------------------)

gen-all:abusiveexperiencereport1 acceleratedmobilepageurl1 accessapproval1_beta1 accesscontextmanager1_beta accesscontextmanager1 adexchangebuyer1d2 adexchangebuyer1d3 adexchangebuyer1d4 adexchangebuyer2_beta1 adexperiencereport1 admin1_datatransfer admin1_directory admin1_reports adsense1d4 adsensehost4d1 alertcenter1_beta1 analytics2d4 analytics3 analyticsreporting4 androiddeviceprovisioning1 androidenterprise1 androidmanagement1 androidpublisher1 androidpublisher1d1 androidpublisher2 androidpublisher3 appengine1_alpha appengine1_beta appengine1 appengine1_beta4 appengine1_beta5 appsactivity1 appstate1 bigquery2 bigquerydatatransfer1 bigtableadmin1 bigtableadmin2 binaryauthorization1_beta1 blogger2 blogger3 books1 calendar3 chat1 civicinfo2 classroom1 cloudasset1 cloudasset1_beta1 cloudbilling1 cloudbuild1_alpha1 cloudbuild1 clouddebugger2 clouderrorreporting1_beta1 cloudfunctions1 cloudfunctions1_beta2 cloudidentity1 cloudidentity1_beta1 cloudiot1_alpha1 cloudiot1 cloudkms1 cloudprivatecatalog1_beta1 cloudprivatecatalogproducer1_beta1 cloudprofiler2 cloudresourcemanager1 cloudresourcemanager1_beta1 cloudresourcemanager2 cloudresourcemanager2_beta1 cloudscheduler1 cloudscheduler1_beta1 cloudsearch1 cloudshell1_alpha1 cloudshell1 cloudtasks2 cloudtasks2_beta2 cloudtasks2_beta3 cloudtrace2_alpha1 cloudtrace1 cloudtrace2 commentanalyzer1_alpha1 composer1 composer1_beta1 computealpha computebeta compute1 container1 container1_beta1 containeranalysis1_alpha1 containeranalysis1_beta1 content2 content2d1 customsearch1 dataflow1_b3 datafusion1_beta1 dataproc1 dataproc1_beta2 datastore1 datastore1_beta1 datastore1_beta3 deploymentmanageralpha deploymentmanager2_beta deploymentmanager2 dfareporting3d1 dfareporting3d2 dfareporting3d3 dialogflow2 dialogflow2_beta1 digitalassetlinks1 discovery1 dlp2 dns1 dns1_beta2 dns2_beta1 docs1 doubleclickbidmanager1 doubleclicksearch2 drive2 drive3 driveactivity2 factchecktools1_alpha1 fcm1 file1 file1_beta1 firebase1_beta1 firebasedynamiclinks1 firebasehosting1_beta1 firebaserules1 firestore1 firestore1_beta1 firestore1_beta2 fitness1 fusiontables1 fusiontables2 games1 gamesConfiguration1_configuration gamesManagement1_management genomics1_alpha2 genomics2_alpha1 genomics1 gmail1 groupsmigration1 groupssettings1 healthcare1_alpha2 healthcare1_beta1 iam1 iamcredentials1 iap1 iap1_beta1 identitytoolkit3 indexing3 jobs3_p1beta1 jobs2 jobs3 kgsearch1 language1 language1_beta1 language1_beta2 libraryagent1 licensing1 logging2 manufacturers1 mirror1 ml1 monitoring3_p1alpha1 monitoring3 oauth1 oauth2 osconfig1_alpha1 osconfig1_alpha2 oslogin1_alpha oslogin1_beta oslogin1 pagespeedonline1 pagespeedonline2 pagespeedonline4 pagespeedonline5 people1 playcustomapp1 plus1 plusDomains1 poly1 proximitybeacon1_beta1 pubsub1_beta1a pubsub1 pubsub1_beta2 redis1 redis1_beta1 remotebuildexecution1_alpha remotebuildexecution1 remotebuildexecution2 replicapool1_beta1 reseller1 run1_alpha1 run1 runtimeconfig1 runtimeconfig1_beta1 safebrowsing4 script1 searchconsole1 securitycenter1_p1alpha1 securitycenter1 securitycenter1_beta1 servicebroker1_alpha1 servicebroker1 servicebroker1_beta1 serviceconsumermanagement1 servicecontrol1 servicemanagement1 servicenetworking1_beta servicenetworking1 serviceusage1 serviceusage1_beta1 sheets4 siteVerification1 slides1 sourcerepo1 spanner1 speech1_p1beta1 speech1 sqladmin1_beta4 storage1 storage1_beta1 storage1_beta2 storagetransfer1 streetviewpublish1 surveys2 tagmanager1 tagmanager2 tasks1 testing1 texttospeech1 texttospeech1_beta1 toolresults1_beta3 tpu1_alpha1 tpu1 translate2 translate3_beta1 urlshortener1 vault1 videointelligence1_p1beta1 videointelligence1_p2beta1 videointelligence1_p3beta1 videointelligence1 videointelligence1_beta2 vision1_p1beta1 vision1_p2beta1 vision1 webfonts1 webmasters3 websecurityscanner1_alpha websecurityscanner1_beta websecurityscanner1 youtube3 youtubeAnalytics1 youtubeAnalytics2 youtubereporting1 

abusiveexperiencereport1: abusiveexperiencereport/v1/spec.json
	$(MCP) generate $< abusiveexperiencereport/v1

acceleratedmobilepageurl1: acceleratedmobilepageurl/v1/spec.json
	$(MCP) generate $< acceleratedmobilepageurl/v1

accessapproval1_beta1: accessapproval/v1beta1/spec.json
	$(MCP) generate $< accessapproval/v1beta1

accesscontextmanager1_beta: accesscontextmanager/v1beta/spec.json
	$(MCP) generate $< accesscontextmanager/v1beta

accesscontextmanager1: accesscontextmanager/v1/spec.json
	$(MCP) generate $< accesscontextmanager/v1

adexchangebuyer1d2: adexchangebuyer/v1.2/spec.json
	$(MCP) generate $< adexchangebuyer/v1.2

adexchangebuyer1d3: adexchangebuyer/v1.3/spec.json
	$(MCP) generate $< adexchangebuyer/v1.3

adexchangebuyer1d4: adexchangebuyer/v1.4/spec.json
	$(MCP) generate $< adexchangebuyer/v1.4

adexchangebuyer2_beta1: adexchangebuyer/v2beta1/spec.json
	$(MCP) generate $< adexchangebuyer/v2beta1

adexperiencereport1: adexperiencereport/v1/spec.json
	$(MCP) generate $< adexperiencereport/v1

admin1_datatransfer: admin/datatransfer_v1/spec.json
	$(MCP) generate $< admin/datatransfer_v1

admin1_directory: admin/directory_v1/spec.json
	$(MCP) generate $< admin/directory_v1

admin1_reports: admin/reports_v1/spec.json
	$(MCP) generate $< admin/reports_v1

adsense1d4: adsense/v1.4/spec.json
	$(MCP) generate $< adsense/v1.4

adsensehost4d1: adsensehost/v4.1/spec.json
	$(MCP) generate $< adsensehost/v4.1

alertcenter1_beta1: alertcenter/v1beta1/spec.json
	$(MCP) generate $< alertcenter/v1beta1

analytics2d4: analytics/v2.4/spec.json
	$(MCP) generate $< analytics/v2.4

analytics3: analytics/v3/spec.json
	$(MCP) generate $< analytics/v3

analyticsreporting4: analyticsreporting/v4/spec.json
	$(MCP) generate $< analyticsreporting/v4

androiddeviceprovisioning1: androiddeviceprovisioning/v1/spec.json
	$(MCP) generate $< androiddeviceprovisioning/v1

androidenterprise1: androidenterprise/v1/spec.json
	$(MCP) generate $< androidenterprise/v1

androidmanagement1: androidmanagement/v1/spec.json
	$(MCP) generate $< androidmanagement/v1

androidpublisher1: androidpublisher/v1/spec.json
	$(MCP) generate $< androidpublisher/v1

androidpublisher1d1: androidpublisher/v1.1/spec.json
	$(MCP) generate $< androidpublisher/v1.1

androidpublisher2: androidpublisher/v2/spec.json
	$(MCP) generate $< androidpublisher/v2

androidpublisher3: androidpublisher/v3/spec.json
	$(MCP) generate $< androidpublisher/v3

appengine1_alpha: appengine/v1alpha/spec.json
	$(MCP) generate $< appengine/v1alpha

appengine1_beta: appengine/v1beta/spec.json
	$(MCP) generate $< appengine/v1beta

appengine1: appengine/v1/spec.json
	$(MCP) generate $< appengine/v1

appengine1_beta4: appengine/v1beta4/spec.json
	$(MCP) generate $< appengine/v1beta4

appengine1_beta5: appengine/v1beta5/spec.json
	$(MCP) generate $< appengine/v1beta5

appsactivity1: appsactivity/v1/spec.json
	$(MCP) generate $< appsactivity/v1

appstate1: appstate/v1/spec.json
	$(MCP) generate $< appstate/v1

bigquery2: bigquery/v2/spec.json
	$(MCP) generate $< bigquery/v2

bigquerydatatransfer1: bigquerydatatransfer/v1/spec.json
	$(MCP) generate $< bigquerydatatransfer/v1

bigtableadmin1: bigtableadmin/v1/spec.json
	$(MCP) generate $< bigtableadmin/v1

bigtableadmin2: bigtableadmin/v2/spec.json
	$(MCP) generate $< bigtableadmin/v2

binaryauthorization1_beta1: binaryauthorization/v1beta1/spec.json
	$(MCP) generate $< binaryauthorization/v1beta1

blogger2: blogger/v2/spec.json
	$(MCP) generate $< blogger/v2

blogger3: blogger/v3/spec.json
	$(MCP) generate $< blogger/v3

books1: books/v1/spec.json
	$(MCP) generate $< books/v1

calendar3: calendar/v3/spec.json
	$(MCP) generate $< calendar/v3

chat1: chat/v1/spec.json
	$(MCP) generate $< chat/v1

civicinfo2: civicinfo/v2/spec.json
	$(MCP) generate $< civicinfo/v2

classroom1: classroom/v1/spec.json
	$(MCP) generate $< classroom/v1

cloudasset1: cloudasset/v1/spec.json
	$(MCP) generate $< cloudasset/v1

cloudasset1_beta1: cloudasset/v1beta1/spec.json
	$(MCP) generate $< cloudasset/v1beta1

cloudbilling1: cloudbilling/v1/spec.json
	$(MCP) generate $< cloudbilling/v1

cloudbuild1_alpha1: cloudbuild/v1alpha1/spec.json
	$(MCP) generate $< cloudbuild/v1alpha1

cloudbuild1: cloudbuild/v1/spec.json
	$(MCP) generate $< cloudbuild/v1

clouddebugger2: clouddebugger/v2/spec.json
	$(MCP) generate $< clouddebugger/v2

clouderrorreporting1_beta1: clouderrorreporting/v1beta1/spec.json
	$(MCP) generate $< clouderrorreporting/v1beta1

cloudfunctions1: cloudfunctions/v1/spec.json
	$(MCP) generate $< cloudfunctions/v1

cloudfunctions1_beta2: cloudfunctions/v1beta2/spec.json
	$(MCP) generate $< cloudfunctions/v1beta2

cloudidentity1: cloudidentity/v1/spec.json
	$(MCP) generate $< cloudidentity/v1

cloudidentity1_beta1: cloudidentity/v1beta1/spec.json
	$(MCP) generate $< cloudidentity/v1beta1

cloudiot1_alpha1: cloudiot/v1alpha1/spec.json
	$(MCP) generate $< cloudiot/v1alpha1

cloudiot1: cloudiot/v1/spec.json
	$(MCP) generate $< cloudiot/v1

cloudkms1: cloudkms/v1/spec.json
	$(MCP) generate $< cloudkms/v1

cloudprivatecatalog1_beta1: cloudprivatecatalog/v1beta1/spec.json
	$(MCP) generate $< cloudprivatecatalog/v1beta1

cloudprivatecatalogproducer1_beta1: cloudprivatecatalogproducer/v1beta1/spec.json
	$(MCP) generate $< cloudprivatecatalogproducer/v1beta1

cloudprofiler2: cloudprofiler/v2/spec.json
	$(MCP) generate $< cloudprofiler/v2

cloudresourcemanager1: cloudresourcemanager/v1/spec.json
	$(MCP) generate $< cloudresourcemanager/v1

cloudresourcemanager1_beta1: cloudresourcemanager/v1beta1/spec.json
	$(MCP) generate $< cloudresourcemanager/v1beta1

cloudresourcemanager2: cloudresourcemanager/v2/spec.json
	$(MCP) generate $< cloudresourcemanager/v2

cloudresourcemanager2_beta1: cloudresourcemanager/v2beta1/spec.json
	$(MCP) generate $< cloudresourcemanager/v2beta1

cloudscheduler1: cloudscheduler/v1/spec.json
	$(MCP) generate $< cloudscheduler/v1

cloudscheduler1_beta1: cloudscheduler/v1beta1/spec.json
	$(MCP) generate $< cloudscheduler/v1beta1

cloudsearch1: cloudsearch/v1/spec.json
	$(MCP) generate $< cloudsearch/v1

cloudshell1_alpha1: cloudshell/v1alpha1/spec.json
	$(MCP) generate $< cloudshell/v1alpha1

cloudshell1: cloudshell/v1/spec.json
	$(MCP) generate $< cloudshell/v1

cloudtasks2: cloudtasks/v2/spec.json
	$(MCP) generate $< cloudtasks/v2

cloudtasks2_beta2: cloudtasks/v2beta2/spec.json
	$(MCP) generate $< cloudtasks/v2beta2

cloudtasks2_beta3: cloudtasks/v2beta3/spec.json
	$(MCP) generate $< cloudtasks/v2beta3

cloudtrace2_alpha1: cloudtrace/v2alpha1/spec.json
	$(MCP) generate $< cloudtrace/v2alpha1

cloudtrace1: cloudtrace/v1/spec.json
	$(MCP) generate $< cloudtrace/v1

cloudtrace2: cloudtrace/v2/spec.json
	$(MCP) generate $< cloudtrace/v2

commentanalyzer1_alpha1: commentanalyzer/v1alpha1/spec.json
	$(MCP) generate $< commentanalyzer/v1alpha1

composer1: composer/v1/spec.json
	$(MCP) generate $< composer/v1

composer1_beta1: composer/v1beta1/spec.json
	$(MCP) generate $< composer/v1beta1

computealpha: compute/alpha/spec.json
	$(MCP) generate $< compute/alpha

computebeta: compute/beta/spec.json
	$(MCP) generate $< compute/beta

compute1: compute/v1/spec.json
	$(MCP) generate $< compute/v1

container1: container/v1/spec.json
	$(MCP) generate $< container/v1

container1_beta1: container/v1beta1/spec.json
	$(MCP) generate $< container/v1beta1

containeranalysis1_alpha1: containeranalysis/v1alpha1/spec.json
	$(MCP) generate $< containeranalysis/v1alpha1

containeranalysis1_beta1: containeranalysis/v1beta1/spec.json
	$(MCP) generate $< containeranalysis/v1beta1

content2: content/v2/spec.json
	$(MCP) generate $< content/v2

content2d1: content/v2.1/spec.json
	$(MCP) generate $< content/v2.1

customsearch1: customsearch/v1/spec.json
	$(MCP) generate $< customsearch/v1

dataflow1_b3: dataflow/v1b3/spec.json
	$(MCP) generate $< dataflow/v1b3

datafusion1_beta1: datafusion/v1beta1/spec.json
	$(MCP) generate $< datafusion/v1beta1

dataproc1: dataproc/v1/spec.json
	$(MCP) generate $< dataproc/v1

dataproc1_beta2: dataproc/v1beta2/spec.json
	$(MCP) generate $< dataproc/v1beta2

datastore1: datastore/v1/spec.json
	$(MCP) generate $< datastore/v1

datastore1_beta1: datastore/v1beta1/spec.json
	$(MCP) generate $< datastore/v1beta1

datastore1_beta3: datastore/v1beta3/spec.json
	$(MCP) generate $< datastore/v1beta3

deploymentmanageralpha: deploymentmanager/alpha/spec.json
	$(MCP) generate $< deploymentmanager/alpha

deploymentmanager2_beta: deploymentmanager/v2beta/spec.json
	$(MCP) generate $< deploymentmanager/v2beta

deploymentmanager2: deploymentmanager/v2/spec.json
	$(MCP) generate $< deploymentmanager/v2

dfareporting3d1: dfareporting/v3.1/spec.json
	$(MCP) generate $< dfareporting/v3.1

dfareporting3d2: dfareporting/v3.2/spec.json
	$(MCP) generate $< dfareporting/v3.2

dfareporting3d3: dfareporting/v3.3/spec.json
	$(MCP) generate $< dfareporting/v3.3

dialogflow2: dialogflow/v2/spec.json
	$(MCP) generate $< dialogflow/v2

dialogflow2_beta1: dialogflow/v2beta1/spec.json
	$(MCP) generate $< dialogflow/v2beta1

digitalassetlinks1: digitalassetlinks/v1/spec.json
	$(MCP) generate $< digitalassetlinks/v1

discovery1: discovery/v1/spec.json
	$(MCP) generate $< discovery/v1

dlp2: dlp/v2/spec.json
	$(MCP) generate $< dlp/v2

dns1: dns/v1/spec.json
	$(MCP) generate $< dns/v1

dns1_beta2: dns/v1beta2/spec.json
	$(MCP) generate $< dns/v1beta2

dns2_beta1: dns/v2beta1/spec.json
	$(MCP) generate $< dns/v2beta1

docs1: docs/v1/spec.json
	$(MCP) generate $< docs/v1

doubleclickbidmanager1: doubleclickbidmanager/v1/spec.json
	$(MCP) generate $< doubleclickbidmanager/v1

doubleclicksearch2: doubleclicksearch/v2/spec.json
	$(MCP) generate $< doubleclicksearch/v2

drive2: drive/v2/spec.json
	$(MCP) generate $< drive/v2

drive3: drive/v3/spec.json
	$(MCP) generate $< drive/v3

driveactivity2: driveactivity/v2/spec.json
	$(MCP) generate $< driveactivity/v2

factchecktools1_alpha1: factchecktools/v1alpha1/spec.json
	$(MCP) generate $< factchecktools/v1alpha1

fcm1: fcm/v1/spec.json
	$(MCP) generate $< fcm/v1

file1: file/v1/spec.json
	$(MCP) generate $< file/v1

file1_beta1: file/v1beta1/spec.json
	$(MCP) generate $< file/v1beta1

firebase1_beta1: firebase/v1beta1/spec.json
	$(MCP) generate $< firebase/v1beta1

firebasedynamiclinks1: firebasedynamiclinks/v1/spec.json
	$(MCP) generate $< firebasedynamiclinks/v1

firebasehosting1_beta1: firebasehosting/v1beta1/spec.json
	$(MCP) generate $< firebasehosting/v1beta1

firebaserules1: firebaserules/v1/spec.json
	$(MCP) generate $< firebaserules/v1

firestore1: firestore/v1/spec.json
	$(MCP) generate $< firestore/v1

firestore1_beta1: firestore/v1beta1/spec.json
	$(MCP) generate $< firestore/v1beta1

firestore1_beta2: firestore/v1beta2/spec.json
	$(MCP) generate $< firestore/v1beta2

fitness1: fitness/v1/spec.json
	$(MCP) generate $< fitness/v1

fusiontables1: fusiontables/v1/spec.json
	$(MCP) generate $< fusiontables/v1

fusiontables2: fusiontables/v2/spec.json
	$(MCP) generate $< fusiontables/v2

games1: games/v1/spec.json
	$(MCP) generate $< games/v1

gamesConfiguration1_configuration: gamesConfiguration/v1configuration/spec.json
	$(MCP) generate $< gamesConfiguration/v1configuration

gamesManagement1_management: gamesManagement/v1management/spec.json
	$(MCP) generate $< gamesManagement/v1management

genomics1_alpha2: genomics/v1alpha2/spec.json
	$(MCP) generate $< genomics/v1alpha2

genomics2_alpha1: genomics/v2alpha1/spec.json
	$(MCP) generate $< genomics/v2alpha1

genomics1: genomics/v1/spec.json
	$(MCP) generate $< genomics/v1

gmail1: gmail/v1/spec.json
	$(MCP) generate $< gmail/v1

groupsmigration1: groupsmigration/v1/spec.json
	$(MCP) generate $< groupsmigration/v1

groupssettings1: groupssettings/v1/spec.json
	$(MCP) generate $< groupssettings/v1

healthcare1_alpha2: healthcare/v1alpha2/spec.json
	$(MCP) generate $< healthcare/v1alpha2

healthcare1_beta1: healthcare/v1beta1/spec.json
	$(MCP) generate $< healthcare/v1beta1

iam1: iam/v1/spec.json
	$(MCP) generate $< iam/v1

iamcredentials1: iamcredentials/v1/spec.json
	$(MCP) generate $< iamcredentials/v1

iap1: iap/v1/spec.json
	$(MCP) generate $< iap/v1

iap1_beta1: iap/v1beta1/spec.json
	$(MCP) generate $< iap/v1beta1

identitytoolkit3: identitytoolkit/v3/spec.json
	$(MCP) generate $< identitytoolkit/v3

indexing3: indexing/v3/spec.json
	$(MCP) generate $< indexing/v3

jobs3_p1beta1: jobs/v3p1beta1/spec.json
	$(MCP) generate $< jobs/v3p1beta1

jobs2: jobs/v2/spec.json
	$(MCP) generate $< jobs/v2

jobs3: jobs/v3/spec.json
	$(MCP) generate $< jobs/v3

kgsearch1: kgsearch/v1/spec.json
	$(MCP) generate $< kgsearch/v1

language1: language/v1/spec.json
	$(MCP) generate $< language/v1

language1_beta1: language/v1beta1/spec.json
	$(MCP) generate $< language/v1beta1

language1_beta2: language/v1beta2/spec.json
	$(MCP) generate $< language/v1beta2

libraryagent1: libraryagent/v1/spec.json
	$(MCP) generate $< libraryagent/v1

licensing1: licensing/v1/spec.json
	$(MCP) generate $< licensing/v1

logging2: logging/v2/spec.json
	$(MCP) generate $< logging/v2

manufacturers1: manufacturers/v1/spec.json
	$(MCP) generate $< manufacturers/v1

mirror1: mirror/v1/spec.json
	$(MCP) generate $< mirror/v1

ml1: ml/v1/spec.json
	$(MCP) generate $< ml/v1

monitoring3_p1alpha1: monitoring/v3p1alpha1/spec.json
	$(MCP) generate $< monitoring/v3p1alpha1

monitoring3: monitoring/v3/spec.json
	$(MCP) generate $< monitoring/v3

oauth1: oauth/v1/spec.json
	$(MCP) generate $< oauth/v1

oauth2: oauth/v2/spec.json
	$(MCP) generate $< oauth/v2

osconfig1_alpha1: osconfig/v1alpha1/spec.json
	$(MCP) generate $< osconfig/v1alpha1

osconfig1_alpha2: osconfig/v1alpha2/spec.json
	$(MCP) generate $< osconfig/v1alpha2

oslogin1_alpha: oslogin/v1alpha/spec.json
	$(MCP) generate $< oslogin/v1alpha

oslogin1_beta: oslogin/v1beta/spec.json
	$(MCP) generate $< oslogin/v1beta

oslogin1: oslogin/v1/spec.json
	$(MCP) generate $< oslogin/v1

pagespeedonline1: pagespeedonline/v1/spec.json
	$(MCP) generate $< pagespeedonline/v1

pagespeedonline2: pagespeedonline/v2/spec.json
	$(MCP) generate $< pagespeedonline/v2

pagespeedonline4: pagespeedonline/v4/spec.json
	$(MCP) generate $< pagespeedonline/v4

pagespeedonline5: pagespeedonline/v5/spec.json
	$(MCP) generate $< pagespeedonline/v5

people1: people/v1/spec.json
	$(MCP) generate $< people/v1

playcustomapp1: playcustomapp/v1/spec.json
	$(MCP) generate $< playcustomapp/v1

plus1: plus/v1/spec.json
	$(MCP) generate $< plus/v1

plusDomains1: plusDomains/v1/spec.json
	$(MCP) generate $< plusDomains/v1

poly1: poly/v1/spec.json
	$(MCP) generate $< poly/v1

proximitybeacon1_beta1: proximitybeacon/v1beta1/spec.json
	$(MCP) generate $< proximitybeacon/v1beta1

pubsub1_beta1a: pubsub/v1beta1a/spec.json
	$(MCP) generate $< pubsub/v1beta1a

pubsub1: pubsub/v1/spec.json
	$(MCP) generate $< pubsub/v1

pubsub1_beta2: pubsub/v1beta2/spec.json
	$(MCP) generate $< pubsub/v1beta2

redis1: redis/v1/spec.json
	$(MCP) generate $< redis/v1

redis1_beta1: redis/v1beta1/spec.json
	$(MCP) generate $< redis/v1beta1

remotebuildexecution1_alpha: remotebuildexecution/v1alpha/spec.json
	$(MCP) generate $< remotebuildexecution/v1alpha

remotebuildexecution1: remotebuildexecution/v1/spec.json
	$(MCP) generate $< remotebuildexecution/v1

remotebuildexecution2: remotebuildexecution/v2/spec.json
	$(MCP) generate $< remotebuildexecution/v2

replicapool1_beta1: replicapool/v1beta1/spec.json
	$(MCP) generate $< replicapool/v1beta1

reseller1: reseller/v1/spec.json
	$(MCP) generate $< reseller/v1

run1_alpha1: run/v1alpha1/spec.json
	$(MCP) generate $< run/v1alpha1

run1: run/v1/spec.json
	$(MCP) generate $< run/v1

runtimeconfig1: runtimeconfig/v1/spec.json
	$(MCP) generate $< runtimeconfig/v1

runtimeconfig1_beta1: runtimeconfig/v1beta1/spec.json
	$(MCP) generate $< runtimeconfig/v1beta1

safebrowsing4: safebrowsing/v4/spec.json
	$(MCP) generate $< safebrowsing/v4

script1: script/v1/spec.json
	$(MCP) generate $< script/v1

searchconsole1: searchconsole/v1/spec.json
	$(MCP) generate $< searchconsole/v1

securitycenter1_p1alpha1: securitycenter/v1p1alpha1/spec.json
	$(MCP) generate $< securitycenter/v1p1alpha1

securitycenter1: securitycenter/v1/spec.json
	$(MCP) generate $< securitycenter/v1

securitycenter1_beta1: securitycenter/v1beta1/spec.json
	$(MCP) generate $< securitycenter/v1beta1

servicebroker1_alpha1: servicebroker/v1alpha1/spec.json
	$(MCP) generate $< servicebroker/v1alpha1

servicebroker1: servicebroker/v1/spec.json
	$(MCP) generate $< servicebroker/v1

servicebroker1_beta1: servicebroker/v1beta1/spec.json
	$(MCP) generate $< servicebroker/v1beta1

serviceconsumermanagement1: serviceconsumermanagement/v1/spec.json
	$(MCP) generate $< serviceconsumermanagement/v1

servicecontrol1: servicecontrol/v1/spec.json
	$(MCP) generate $< servicecontrol/v1

servicemanagement1: servicemanagement/v1/spec.json
	$(MCP) generate $< servicemanagement/v1

servicenetworking1_beta: servicenetworking/v1beta/spec.json
	$(MCP) generate $< servicenetworking/v1beta

servicenetworking1: servicenetworking/v1/spec.json
	$(MCP) generate $< servicenetworking/v1

serviceusage1: serviceusage/v1/spec.json
	$(MCP) generate $< serviceusage/v1

serviceusage1_beta1: serviceusage/v1beta1/spec.json
	$(MCP) generate $< serviceusage/v1beta1

sheets4: sheets/v4/spec.json
	$(MCP) generate $< sheets/v4

siteVerification1: siteVerification/v1/spec.json
	$(MCP) generate $< siteVerification/v1

slides1: slides/v1/spec.json
	$(MCP) generate $< slides/v1

sourcerepo1: sourcerepo/v1/spec.json
	$(MCP) generate $< sourcerepo/v1

spanner1: spanner/v1/spec.json
	$(MCP) generate $< spanner/v1

speech1_p1beta1: speech/v1p1beta1/spec.json
	$(MCP) generate $< speech/v1p1beta1

speech1: speech/v1/spec.json
	$(MCP) generate $< speech/v1

sqladmin1_beta4: sqladmin/v1beta4/spec.json
	$(MCP) generate $< sqladmin/v1beta4

storage1: storage/v1/spec.json
	$(MCP) generate $< storage/v1

storage1_beta1: storage/v1beta1/spec.json
	$(MCP) generate $< storage/v1beta1

storage1_beta2: storage/v1beta2/spec.json
	$(MCP) generate $< storage/v1beta2

storagetransfer1: storagetransfer/v1/spec.json
	$(MCP) generate $< storagetransfer/v1

streetviewpublish1: streetviewpublish/v1/spec.json
	$(MCP) generate $< streetviewpublish/v1

surveys2: surveys/v2/spec.json
	$(MCP) generate $< surveys/v2

tagmanager1: tagmanager/v1/spec.json
	$(MCP) generate $< tagmanager/v1

tagmanager2: tagmanager/v2/spec.json
	$(MCP) generate $< tagmanager/v2

tasks1: tasks/v1/spec.json
	$(MCP) generate $< tasks/v1

testing1: testing/v1/spec.json
	$(MCP) generate $< testing/v1

texttospeech1: texttospeech/v1/spec.json
	$(MCP) generate $< texttospeech/v1

texttospeech1_beta1: texttospeech/v1beta1/spec.json
	$(MCP) generate $< texttospeech/v1beta1

toolresults1_beta3: toolresults/v1beta3/spec.json
	$(MCP) generate $< toolresults/v1beta3

tpu1_alpha1: tpu/v1alpha1/spec.json
	$(MCP) generate $< tpu/v1alpha1

tpu1: tpu/v1/spec.json
	$(MCP) generate $< tpu/v1

translate2: translate/v2/spec.json
	$(MCP) generate $< translate/v2

translate3_beta1: translate/v3beta1/spec.json
	$(MCP) generate $< translate/v3beta1

urlshortener1: urlshortener/v1/spec.json
	$(MCP) generate $< urlshortener/v1

vault1: vault/v1/spec.json
	$(MCP) generate $< vault/v1

videointelligence1_p1beta1: videointelligence/v1p1beta1/spec.json
	$(MCP) generate $< videointelligence/v1p1beta1

videointelligence1_p2beta1: videointelligence/v1p2beta1/spec.json
	$(MCP) generate $< videointelligence/v1p2beta1

videointelligence1_p3beta1: videointelligence/v1p3beta1/spec.json
	$(MCP) generate $< videointelligence/v1p3beta1

videointelligence1: videointelligence/v1/spec.json
	$(MCP) generate $< videointelligence/v1

videointelligence1_beta2: videointelligence/v1beta2/spec.json
	$(MCP) generate $< videointelligence/v1beta2

vision1_p1beta1: vision/v1p1beta1/spec.json
	$(MCP) generate $< vision/v1p1beta1

vision1_p2beta1: vision/v1p2beta1/spec.json
	$(MCP) generate $< vision/v1p2beta1

vision1: vision/v1/spec.json
	$(MCP) generate $< vision/v1

webfonts1: webfonts/v1/spec.json
	$(MCP) generate $< webfonts/v1

webmasters3: webmasters/v3/spec.json
	$(MCP) generate $< webmasters/v3

websecurityscanner1_alpha: websecurityscanner/v1alpha/spec.json
	$(MCP) generate $< websecurityscanner/v1alpha

websecurityscanner1_beta: websecurityscanner/v1beta/spec.json
	$(MCP) generate $< websecurityscanner/v1beta

websecurityscanner1: websecurityscanner/v1/spec.json
	$(MCP) generate $< websecurityscanner/v1

youtube3: youtube/v3/spec.json
	$(MCP) generate $< youtube/v3

youtubeAnalytics1: youtubeAnalytics/v1/spec.json
	$(MCP) generate $< youtubeAnalytics/v1

youtubeAnalytics2: youtubeAnalytics/v2/spec.json
	$(MCP) generate $< youtubeAnalytics/v2

youtubereporting1: youtubereporting/v1/spec.json
	$(MCP) generate $< youtubereporting/v1
