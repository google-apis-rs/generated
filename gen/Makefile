MCP ?= ../_generator/target/release/mcp

help:
	$(info -- Target ----------------------------------------------------------------------------)
	$(info gen-all             | generate all libraries and CLIs)
	$(info cargo-all           | pass args with ARGS="check" for example)
	$(info --------------------------------------------------------------------------------------)

gen-all: abusiveexperiencereport1 acceleratedmobilepageurl1 adexchangebuyer1d2 adexchangebuyer1d3 adexchangebuyer1d4 adexperiencereport1 admin1_datatransfer admin1_reports analyticsreporting4 androidenterprise1 androidpublisher1 androidpublisher1d1 appstate1 bigtableadmin1 blogger3 cloudasset1 cloudresourcemanager1 cloudresourcemanager1_beta1 cloudshell1 commentanalyzer1_alpha1 computebeta compute1 content2 content2d1 datastore1_beta1 datastore1_beta3 deploymentmanageralpha deploymentmanager2_beta deploymentmanager2 digitalassetlinks1 dns1 dns1_beta2 dns2_beta1 docs1 doubleclickbidmanager1 drive3 factchecktools1_alpha1 firebasedynamiclinks1 gamesManagement1_management genomics1_alpha2 genomics1 groupssettings1 iap1 iap1_beta1 indexing3 kgsearch1 language1 language1_beta1 language1_beta2 licensing1 pagespeedonline1 pagespeedonline2 pagespeedonline4 pagespeedonline5 plus1 plusDomains1 pubsub1_beta1a remotebuildexecution2 replicapool1_beta1 reseller1 runtimeconfig1 servicebroker1 servicecontrol1 siteVerification1 sqladmin1_beta4 storage1_beta1 storagetransfer1 streetviewpublish1 surveys2 texttospeech1 texttospeech1_beta1 urlshortener1 videointelligence1_p1beta1 videointelligence1_p2beta1 videointelligence1_p3beta1 videointelligence1_beta2 webmasters3 youtubeAnalytics1 youtubeAnalytics2 
	$(MAKE) -C .. update-mapped-index
cargo-all: abusiveexperiencereport1-cargo acceleratedmobilepageurl1-cargo adexchangebuyer1d2-cargo adexchangebuyer1d3-cargo adexchangebuyer1d4-cargo adexperiencereport1-cargo admin1_datatransfer-cargo admin1_reports-cargo analyticsreporting4-cargo androidenterprise1-cargo androidpublisher1-cargo androidpublisher1d1-cargo appstate1-cargo bigtableadmin1-cargo blogger3-cargo cloudasset1-cargo cloudresourcemanager1-cargo cloudresourcemanager1_beta1-cargo cloudshell1-cargo commentanalyzer1_alpha1-cargo computebeta-cargo compute1-cargo content2-cargo content2d1-cargo datastore1_beta1-cargo datastore1_beta3-cargo deploymentmanageralpha-cargo deploymentmanager2_beta-cargo deploymentmanager2-cargo digitalassetlinks1-cargo dns1-cargo dns1_beta2-cargo dns2_beta1-cargo docs1-cargo doubleclickbidmanager1-cargo drive3-cargo factchecktools1_alpha1-cargo firebasedynamiclinks1-cargo gamesManagement1_management-cargo genomics1_alpha2-cargo genomics1-cargo groupssettings1-cargo iap1-cargo iap1_beta1-cargo indexing3-cargo kgsearch1-cargo language1-cargo language1_beta1-cargo language1_beta2-cargo licensing1-cargo pagespeedonline1-cargo pagespeedonline2-cargo pagespeedonline4-cargo pagespeedonline5-cargo plus1-cargo plusDomains1-cargo pubsub1_beta1a-cargo remotebuildexecution2-cargo replicapool1_beta1-cargo reseller1-cargo runtimeconfig1-cargo servicebroker1-cargo servicecontrol1-cargo siteVerification1-cargo sqladmin1_beta4-cargo storage1_beta1-cargo storagetransfer1-cargo streetviewpublish1-cargo surveys2-cargo texttospeech1-cargo texttospeech1_beta1-cargo urlshortener1-cargo videointelligence1_p1beta1-cargo videointelligence1_p2beta1-cargo videointelligence1_p3beta1-cargo videointelligence1_beta2-cargo webmasters3-cargo youtubeAnalytics1-cargo youtubeAnalytics2-cargo 


abusiveexperiencereport/v1/lib/Cargo.toml: abusiveexperiencereport/v1/spec.json $(MCP)
	$(MCP) generate $< abusiveexperiencereport/v1 2>abusiveexperiencereport/v1/generator-errors.log && rm abusiveexperiencereport/v1/generator-errors.log
abusiveexperiencereport1: abusiveexperiencereport/v1/lib/Cargo.toml
abusiveexperiencereport1-cargo: abusiveexperiencereport1
	( cd abusiveexperiencereport/v1/lib && cargo $(ARGS) ) 2> abusiveexperiencereport/v1/generator-errors.log && rm abusiveexperiencereport/v1/generator-errors.log

acceleratedmobilepageurl/v1/lib/Cargo.toml: acceleratedmobilepageurl/v1/spec.json $(MCP)
	$(MCP) generate $< acceleratedmobilepageurl/v1 2>acceleratedmobilepageurl/v1/generator-errors.log && rm acceleratedmobilepageurl/v1/generator-errors.log
acceleratedmobilepageurl1: acceleratedmobilepageurl/v1/lib/Cargo.toml
acceleratedmobilepageurl1-cargo: acceleratedmobilepageurl1
	( cd acceleratedmobilepageurl/v1/lib && cargo $(ARGS) ) 2> acceleratedmobilepageurl/v1/generator-errors.log && rm acceleratedmobilepageurl/v1/generator-errors.log

adexchangebuyer/v1.2/lib/Cargo.toml: adexchangebuyer/v1.2/spec.json $(MCP)
	$(MCP) generate $< adexchangebuyer/v1.2 2>adexchangebuyer/v1.2/generator-errors.log && rm adexchangebuyer/v1.2/generator-errors.log
adexchangebuyer1d2: adexchangebuyer/v1.2/lib/Cargo.toml
adexchangebuyer1d2-cargo: adexchangebuyer1d2
	( cd adexchangebuyer/v1.2/lib && cargo $(ARGS) ) 2> adexchangebuyer/v1.2/generator-errors.log && rm adexchangebuyer/v1.2/generator-errors.log

adexchangebuyer/v1.3/lib/Cargo.toml: adexchangebuyer/v1.3/spec.json $(MCP)
	$(MCP) generate $< adexchangebuyer/v1.3 2>adexchangebuyer/v1.3/generator-errors.log && rm adexchangebuyer/v1.3/generator-errors.log
adexchangebuyer1d3: adexchangebuyer/v1.3/lib/Cargo.toml
adexchangebuyer1d3-cargo: adexchangebuyer1d3
	( cd adexchangebuyer/v1.3/lib && cargo $(ARGS) ) 2> adexchangebuyer/v1.3/generator-errors.log && rm adexchangebuyer/v1.3/generator-errors.log

adexchangebuyer/v1.4/lib/Cargo.toml: adexchangebuyer/v1.4/spec.json $(MCP)
	$(MCP) generate $< adexchangebuyer/v1.4 2>adexchangebuyer/v1.4/generator-errors.log && rm adexchangebuyer/v1.4/generator-errors.log
adexchangebuyer1d4: adexchangebuyer/v1.4/lib/Cargo.toml
adexchangebuyer1d4-cargo: adexchangebuyer1d4
	( cd adexchangebuyer/v1.4/lib && cargo $(ARGS) ) 2> adexchangebuyer/v1.4/generator-errors.log && rm adexchangebuyer/v1.4/generator-errors.log

adexperiencereport/v1/lib/Cargo.toml: adexperiencereport/v1/spec.json $(MCP)
	$(MCP) generate $< adexperiencereport/v1 2>adexperiencereport/v1/generator-errors.log && rm adexperiencereport/v1/generator-errors.log
adexperiencereport1: adexperiencereport/v1/lib/Cargo.toml
adexperiencereport1-cargo: adexperiencereport1
	( cd adexperiencereport/v1/lib && cargo $(ARGS) ) 2> adexperiencereport/v1/generator-errors.log && rm adexperiencereport/v1/generator-errors.log

admin/datatransfer_v1/lib/Cargo.toml: admin/datatransfer_v1/spec.json $(MCP)
	$(MCP) generate $< admin/datatransfer_v1 2>admin/datatransfer_v1/generator-errors.log && rm admin/datatransfer_v1/generator-errors.log
admin1_datatransfer: admin/datatransfer_v1/lib/Cargo.toml
admin1_datatransfer-cargo: admin1_datatransfer
	( cd admin/datatransfer_v1/lib && cargo $(ARGS) ) 2> admin/datatransfer_v1/generator-errors.log && rm admin/datatransfer_v1/generator-errors.log

admin/reports_v1/lib/Cargo.toml: admin/reports_v1/spec.json $(MCP)
	$(MCP) generate $< admin/reports_v1 2>admin/reports_v1/generator-errors.log && rm admin/reports_v1/generator-errors.log
admin1_reports: admin/reports_v1/lib/Cargo.toml
admin1_reports-cargo: admin1_reports
	( cd admin/reports_v1/lib && cargo $(ARGS) ) 2> admin/reports_v1/generator-errors.log && rm admin/reports_v1/generator-errors.log

analyticsreporting/v4/lib/Cargo.toml: analyticsreporting/v4/spec.json $(MCP)
	$(MCP) generate $< analyticsreporting/v4 2>analyticsreporting/v4/generator-errors.log && rm analyticsreporting/v4/generator-errors.log
analyticsreporting4: analyticsreporting/v4/lib/Cargo.toml
analyticsreporting4-cargo: analyticsreporting4
	( cd analyticsreporting/v4/lib && cargo $(ARGS) ) 2> analyticsreporting/v4/generator-errors.log && rm analyticsreporting/v4/generator-errors.log

androidenterprise/v1/lib/Cargo.toml: androidenterprise/v1/spec.json $(MCP)
	$(MCP) generate $< androidenterprise/v1 2>androidenterprise/v1/generator-errors.log && rm androidenterprise/v1/generator-errors.log
androidenterprise1: androidenterprise/v1/lib/Cargo.toml
androidenterprise1-cargo: androidenterprise1
	( cd androidenterprise/v1/lib && cargo $(ARGS) ) 2> androidenterprise/v1/generator-errors.log && rm androidenterprise/v1/generator-errors.log

androidpublisher/v1/lib/Cargo.toml: androidpublisher/v1/spec.json $(MCP)
	$(MCP) generate $< androidpublisher/v1 2>androidpublisher/v1/generator-errors.log && rm androidpublisher/v1/generator-errors.log
androidpublisher1: androidpublisher/v1/lib/Cargo.toml
androidpublisher1-cargo: androidpublisher1
	( cd androidpublisher/v1/lib && cargo $(ARGS) ) 2> androidpublisher/v1/generator-errors.log && rm androidpublisher/v1/generator-errors.log

androidpublisher/v1.1/lib/Cargo.toml: androidpublisher/v1.1/spec.json $(MCP)
	$(MCP) generate $< androidpublisher/v1.1 2>androidpublisher/v1.1/generator-errors.log && rm androidpublisher/v1.1/generator-errors.log
androidpublisher1d1: androidpublisher/v1.1/lib/Cargo.toml
androidpublisher1d1-cargo: androidpublisher1d1
	( cd androidpublisher/v1.1/lib && cargo $(ARGS) ) 2> androidpublisher/v1.1/generator-errors.log && rm androidpublisher/v1.1/generator-errors.log

appstate/v1/lib/Cargo.toml: appstate/v1/spec.json $(MCP)
	$(MCP) generate $< appstate/v1 2>appstate/v1/generator-errors.log && rm appstate/v1/generator-errors.log
appstate1: appstate/v1/lib/Cargo.toml
appstate1-cargo: appstate1
	( cd appstate/v1/lib && cargo $(ARGS) ) 2> appstate/v1/generator-errors.log && rm appstate/v1/generator-errors.log

bigtableadmin/v1/lib/Cargo.toml: bigtableadmin/v1/spec.json $(MCP)
	$(MCP) generate $< bigtableadmin/v1 2>bigtableadmin/v1/generator-errors.log && rm bigtableadmin/v1/generator-errors.log
bigtableadmin1: bigtableadmin/v1/lib/Cargo.toml
bigtableadmin1-cargo: bigtableadmin1
	( cd bigtableadmin/v1/lib && cargo $(ARGS) ) 2> bigtableadmin/v1/generator-errors.log && rm bigtableadmin/v1/generator-errors.log

blogger/v3/lib/Cargo.toml: blogger/v3/spec.json $(MCP)
	$(MCP) generate $< blogger/v3 2>blogger/v3/generator-errors.log && rm blogger/v3/generator-errors.log
blogger3: blogger/v3/lib/Cargo.toml
blogger3-cargo: blogger3
	( cd blogger/v3/lib && cargo $(ARGS) ) 2> blogger/v3/generator-errors.log && rm blogger/v3/generator-errors.log

cloudasset/v1/lib/Cargo.toml: cloudasset/v1/spec.json $(MCP)
	$(MCP) generate $< cloudasset/v1 2>cloudasset/v1/generator-errors.log && rm cloudasset/v1/generator-errors.log
cloudasset1: cloudasset/v1/lib/Cargo.toml
cloudasset1-cargo: cloudasset1
	( cd cloudasset/v1/lib && cargo $(ARGS) ) 2> cloudasset/v1/generator-errors.log && rm cloudasset/v1/generator-errors.log

cloudresourcemanager/v1/lib/Cargo.toml: cloudresourcemanager/v1/spec.json $(MCP)
	$(MCP) generate $< cloudresourcemanager/v1 2>cloudresourcemanager/v1/generator-errors.log && rm cloudresourcemanager/v1/generator-errors.log
cloudresourcemanager1: cloudresourcemanager/v1/lib/Cargo.toml
cloudresourcemanager1-cargo: cloudresourcemanager1
	( cd cloudresourcemanager/v1/lib && cargo $(ARGS) ) 2> cloudresourcemanager/v1/generator-errors.log && rm cloudresourcemanager/v1/generator-errors.log

cloudresourcemanager/v1beta1/lib/Cargo.toml: cloudresourcemanager/v1beta1/spec.json $(MCP)
	$(MCP) generate $< cloudresourcemanager/v1beta1 2>cloudresourcemanager/v1beta1/generator-errors.log && rm cloudresourcemanager/v1beta1/generator-errors.log
cloudresourcemanager1_beta1: cloudresourcemanager/v1beta1/lib/Cargo.toml
cloudresourcemanager1_beta1-cargo: cloudresourcemanager1_beta1
	( cd cloudresourcemanager/v1beta1/lib && cargo $(ARGS) ) 2> cloudresourcemanager/v1beta1/generator-errors.log && rm cloudresourcemanager/v1beta1/generator-errors.log

cloudshell/v1/lib/Cargo.toml: cloudshell/v1/spec.json $(MCP)
	$(MCP) generate $< cloudshell/v1 2>cloudshell/v1/generator-errors.log && rm cloudshell/v1/generator-errors.log
cloudshell1: cloudshell/v1/lib/Cargo.toml
cloudshell1-cargo: cloudshell1
	( cd cloudshell/v1/lib && cargo $(ARGS) ) 2> cloudshell/v1/generator-errors.log && rm cloudshell/v1/generator-errors.log

commentanalyzer/v1alpha1/lib/Cargo.toml: commentanalyzer/v1alpha1/spec.json $(MCP)
	$(MCP) generate $< commentanalyzer/v1alpha1 2>commentanalyzer/v1alpha1/generator-errors.log && rm commentanalyzer/v1alpha1/generator-errors.log
commentanalyzer1_alpha1: commentanalyzer/v1alpha1/lib/Cargo.toml
commentanalyzer1_alpha1-cargo: commentanalyzer1_alpha1
	( cd commentanalyzer/v1alpha1/lib && cargo $(ARGS) ) 2> commentanalyzer/v1alpha1/generator-errors.log && rm commentanalyzer/v1alpha1/generator-errors.log

compute/beta/lib/Cargo.toml: compute/beta/spec.json $(MCP)
	$(MCP) generate $< compute/beta 2>compute/beta/generator-errors.log && rm compute/beta/generator-errors.log
computebeta: compute/beta/lib/Cargo.toml
computebeta-cargo: computebeta
	( cd compute/beta/lib && cargo $(ARGS) ) 2> compute/beta/generator-errors.log && rm compute/beta/generator-errors.log

compute/v1/lib/Cargo.toml: compute/v1/spec.json $(MCP)
	$(MCP) generate $< compute/v1 2>compute/v1/generator-errors.log && rm compute/v1/generator-errors.log
compute1: compute/v1/lib/Cargo.toml
compute1-cargo: compute1
	( cd compute/v1/lib && cargo $(ARGS) ) 2> compute/v1/generator-errors.log && rm compute/v1/generator-errors.log

content/v2/lib/Cargo.toml: content/v2/spec.json $(MCP)
	$(MCP) generate $< content/v2 2>content/v2/generator-errors.log && rm content/v2/generator-errors.log
content2: content/v2/lib/Cargo.toml
content2-cargo: content2
	( cd content/v2/lib && cargo $(ARGS) ) 2> content/v2/generator-errors.log && rm content/v2/generator-errors.log

content/v2.1/lib/Cargo.toml: content/v2.1/spec.json $(MCP)
	$(MCP) generate $< content/v2.1 2>content/v2.1/generator-errors.log && rm content/v2.1/generator-errors.log
content2d1: content/v2.1/lib/Cargo.toml
content2d1-cargo: content2d1
	( cd content/v2.1/lib && cargo $(ARGS) ) 2> content/v2.1/generator-errors.log && rm content/v2.1/generator-errors.log

datastore/v1beta1/lib/Cargo.toml: datastore/v1beta1/spec.json $(MCP)
	$(MCP) generate $< datastore/v1beta1 2>datastore/v1beta1/generator-errors.log && rm datastore/v1beta1/generator-errors.log
datastore1_beta1: datastore/v1beta1/lib/Cargo.toml
datastore1_beta1-cargo: datastore1_beta1
	( cd datastore/v1beta1/lib && cargo $(ARGS) ) 2> datastore/v1beta1/generator-errors.log && rm datastore/v1beta1/generator-errors.log

datastore/v1beta3/lib/Cargo.toml: datastore/v1beta3/spec.json $(MCP)
	$(MCP) generate $< datastore/v1beta3 2>datastore/v1beta3/generator-errors.log && rm datastore/v1beta3/generator-errors.log
datastore1_beta3: datastore/v1beta3/lib/Cargo.toml
datastore1_beta3-cargo: datastore1_beta3
	( cd datastore/v1beta3/lib && cargo $(ARGS) ) 2> datastore/v1beta3/generator-errors.log && rm datastore/v1beta3/generator-errors.log

deploymentmanager/alpha/lib/Cargo.toml: deploymentmanager/alpha/spec.json $(MCP)
	$(MCP) generate $< deploymentmanager/alpha 2>deploymentmanager/alpha/generator-errors.log && rm deploymentmanager/alpha/generator-errors.log
deploymentmanageralpha: deploymentmanager/alpha/lib/Cargo.toml
deploymentmanageralpha-cargo: deploymentmanageralpha
	( cd deploymentmanager/alpha/lib && cargo $(ARGS) ) 2> deploymentmanager/alpha/generator-errors.log && rm deploymentmanager/alpha/generator-errors.log

deploymentmanager/v2beta/lib/Cargo.toml: deploymentmanager/v2beta/spec.json $(MCP)
	$(MCP) generate $< deploymentmanager/v2beta 2>deploymentmanager/v2beta/generator-errors.log && rm deploymentmanager/v2beta/generator-errors.log
deploymentmanager2_beta: deploymentmanager/v2beta/lib/Cargo.toml
deploymentmanager2_beta-cargo: deploymentmanager2_beta
	( cd deploymentmanager/v2beta/lib && cargo $(ARGS) ) 2> deploymentmanager/v2beta/generator-errors.log && rm deploymentmanager/v2beta/generator-errors.log

deploymentmanager/v2/lib/Cargo.toml: deploymentmanager/v2/spec.json $(MCP)
	$(MCP) generate $< deploymentmanager/v2 2>deploymentmanager/v2/generator-errors.log && rm deploymentmanager/v2/generator-errors.log
deploymentmanager2: deploymentmanager/v2/lib/Cargo.toml
deploymentmanager2-cargo: deploymentmanager2
	( cd deploymentmanager/v2/lib && cargo $(ARGS) ) 2> deploymentmanager/v2/generator-errors.log && rm deploymentmanager/v2/generator-errors.log

digitalassetlinks/v1/lib/Cargo.toml: digitalassetlinks/v1/spec.json $(MCP)
	$(MCP) generate $< digitalassetlinks/v1 2>digitalassetlinks/v1/generator-errors.log && rm digitalassetlinks/v1/generator-errors.log
digitalassetlinks1: digitalassetlinks/v1/lib/Cargo.toml
digitalassetlinks1-cargo: digitalassetlinks1
	( cd digitalassetlinks/v1/lib && cargo $(ARGS) ) 2> digitalassetlinks/v1/generator-errors.log && rm digitalassetlinks/v1/generator-errors.log

dns/v1/lib/Cargo.toml: dns/v1/spec.json $(MCP)
	$(MCP) generate $< dns/v1 2>dns/v1/generator-errors.log && rm dns/v1/generator-errors.log
dns1: dns/v1/lib/Cargo.toml
dns1-cargo: dns1
	( cd dns/v1/lib && cargo $(ARGS) ) 2> dns/v1/generator-errors.log && rm dns/v1/generator-errors.log

dns/v1beta2/lib/Cargo.toml: dns/v1beta2/spec.json $(MCP)
	$(MCP) generate $< dns/v1beta2 2>dns/v1beta2/generator-errors.log && rm dns/v1beta2/generator-errors.log
dns1_beta2: dns/v1beta2/lib/Cargo.toml
dns1_beta2-cargo: dns1_beta2
	( cd dns/v1beta2/lib && cargo $(ARGS) ) 2> dns/v1beta2/generator-errors.log && rm dns/v1beta2/generator-errors.log

dns/v2beta1/lib/Cargo.toml: dns/v2beta1/spec.json $(MCP)
	$(MCP) generate $< dns/v2beta1 2>dns/v2beta1/generator-errors.log && rm dns/v2beta1/generator-errors.log
dns2_beta1: dns/v2beta1/lib/Cargo.toml
dns2_beta1-cargo: dns2_beta1
	( cd dns/v2beta1/lib && cargo $(ARGS) ) 2> dns/v2beta1/generator-errors.log && rm dns/v2beta1/generator-errors.log

docs/v1/lib/Cargo.toml: docs/v1/spec.json $(MCP)
	$(MCP) generate $< docs/v1 2>docs/v1/generator-errors.log && rm docs/v1/generator-errors.log
docs1: docs/v1/lib/Cargo.toml
docs1-cargo: docs1
	( cd docs/v1/lib && cargo $(ARGS) ) 2> docs/v1/generator-errors.log && rm docs/v1/generator-errors.log

doubleclickbidmanager/v1/lib/Cargo.toml: doubleclickbidmanager/v1/spec.json $(MCP)
	$(MCP) generate $< doubleclickbidmanager/v1 2>doubleclickbidmanager/v1/generator-errors.log && rm doubleclickbidmanager/v1/generator-errors.log
doubleclickbidmanager1: doubleclickbidmanager/v1/lib/Cargo.toml
doubleclickbidmanager1-cargo: doubleclickbidmanager1
	( cd doubleclickbidmanager/v1/lib && cargo $(ARGS) ) 2> doubleclickbidmanager/v1/generator-errors.log && rm doubleclickbidmanager/v1/generator-errors.log

drive/v3/lib/Cargo.toml: drive/v3/spec.json $(MCP)
	$(MCP) generate $< drive/v3 2>drive/v3/generator-errors.log && rm drive/v3/generator-errors.log
drive3: drive/v3/lib/Cargo.toml
drive3-cargo: drive3
	( cd drive/v3/lib && cargo $(ARGS) ) 2> drive/v3/generator-errors.log && rm drive/v3/generator-errors.log

factchecktools/v1alpha1/lib/Cargo.toml: factchecktools/v1alpha1/spec.json $(MCP)
	$(MCP) generate $< factchecktools/v1alpha1 2>factchecktools/v1alpha1/generator-errors.log && rm factchecktools/v1alpha1/generator-errors.log
factchecktools1_alpha1: factchecktools/v1alpha1/lib/Cargo.toml
factchecktools1_alpha1-cargo: factchecktools1_alpha1
	( cd factchecktools/v1alpha1/lib && cargo $(ARGS) ) 2> factchecktools/v1alpha1/generator-errors.log && rm factchecktools/v1alpha1/generator-errors.log

firebasedynamiclinks/v1/lib/Cargo.toml: firebasedynamiclinks/v1/spec.json $(MCP)
	$(MCP) generate $< firebasedynamiclinks/v1 2>firebasedynamiclinks/v1/generator-errors.log && rm firebasedynamiclinks/v1/generator-errors.log
firebasedynamiclinks1: firebasedynamiclinks/v1/lib/Cargo.toml
firebasedynamiclinks1-cargo: firebasedynamiclinks1
	( cd firebasedynamiclinks/v1/lib && cargo $(ARGS) ) 2> firebasedynamiclinks/v1/generator-errors.log && rm firebasedynamiclinks/v1/generator-errors.log

gamesManagement/v1management/lib/Cargo.toml: gamesManagement/v1management/spec.json $(MCP)
	$(MCP) generate $< gamesManagement/v1management 2>gamesManagement/v1management/generator-errors.log && rm gamesManagement/v1management/generator-errors.log
gamesManagement1_management: gamesManagement/v1management/lib/Cargo.toml
gamesManagement1_management-cargo: gamesManagement1_management
	( cd gamesManagement/v1management/lib && cargo $(ARGS) ) 2> gamesManagement/v1management/generator-errors.log && rm gamesManagement/v1management/generator-errors.log

genomics/v1alpha2/lib/Cargo.toml: genomics/v1alpha2/spec.json $(MCP)
	$(MCP) generate $< genomics/v1alpha2 2>genomics/v1alpha2/generator-errors.log && rm genomics/v1alpha2/generator-errors.log
genomics1_alpha2: genomics/v1alpha2/lib/Cargo.toml
genomics1_alpha2-cargo: genomics1_alpha2
	( cd genomics/v1alpha2/lib && cargo $(ARGS) ) 2> genomics/v1alpha2/generator-errors.log && rm genomics/v1alpha2/generator-errors.log

genomics/v1/lib/Cargo.toml: genomics/v1/spec.json $(MCP)
	$(MCP) generate $< genomics/v1 2>genomics/v1/generator-errors.log && rm genomics/v1/generator-errors.log
genomics1: genomics/v1/lib/Cargo.toml
genomics1-cargo: genomics1
	( cd genomics/v1/lib && cargo $(ARGS) ) 2> genomics/v1/generator-errors.log && rm genomics/v1/generator-errors.log

groupssettings/v1/lib/Cargo.toml: groupssettings/v1/spec.json $(MCP)
	$(MCP) generate $< groupssettings/v1 2>groupssettings/v1/generator-errors.log && rm groupssettings/v1/generator-errors.log
groupssettings1: groupssettings/v1/lib/Cargo.toml
groupssettings1-cargo: groupssettings1
	( cd groupssettings/v1/lib && cargo $(ARGS) ) 2> groupssettings/v1/generator-errors.log && rm groupssettings/v1/generator-errors.log

iap/v1/lib/Cargo.toml: iap/v1/spec.json $(MCP)
	$(MCP) generate $< iap/v1 2>iap/v1/generator-errors.log && rm iap/v1/generator-errors.log
iap1: iap/v1/lib/Cargo.toml
iap1-cargo: iap1
	( cd iap/v1/lib && cargo $(ARGS) ) 2> iap/v1/generator-errors.log && rm iap/v1/generator-errors.log

iap/v1beta1/lib/Cargo.toml: iap/v1beta1/spec.json $(MCP)
	$(MCP) generate $< iap/v1beta1 2>iap/v1beta1/generator-errors.log && rm iap/v1beta1/generator-errors.log
iap1_beta1: iap/v1beta1/lib/Cargo.toml
iap1_beta1-cargo: iap1_beta1
	( cd iap/v1beta1/lib && cargo $(ARGS) ) 2> iap/v1beta1/generator-errors.log && rm iap/v1beta1/generator-errors.log

indexing/v3/lib/Cargo.toml: indexing/v3/spec.json $(MCP)
	$(MCP) generate $< indexing/v3 2>indexing/v3/generator-errors.log && rm indexing/v3/generator-errors.log
indexing3: indexing/v3/lib/Cargo.toml
indexing3-cargo: indexing3
	( cd indexing/v3/lib && cargo $(ARGS) ) 2> indexing/v3/generator-errors.log && rm indexing/v3/generator-errors.log

kgsearch/v1/lib/Cargo.toml: kgsearch/v1/spec.json $(MCP)
	$(MCP) generate $< kgsearch/v1 2>kgsearch/v1/generator-errors.log && rm kgsearch/v1/generator-errors.log
kgsearch1: kgsearch/v1/lib/Cargo.toml
kgsearch1-cargo: kgsearch1
	( cd kgsearch/v1/lib && cargo $(ARGS) ) 2> kgsearch/v1/generator-errors.log && rm kgsearch/v1/generator-errors.log

language/v1/lib/Cargo.toml: language/v1/spec.json $(MCP)
	$(MCP) generate $< language/v1 2>language/v1/generator-errors.log && rm language/v1/generator-errors.log
language1: language/v1/lib/Cargo.toml
language1-cargo: language1
	( cd language/v1/lib && cargo $(ARGS) ) 2> language/v1/generator-errors.log && rm language/v1/generator-errors.log

language/v1beta1/lib/Cargo.toml: language/v1beta1/spec.json $(MCP)
	$(MCP) generate $< language/v1beta1 2>language/v1beta1/generator-errors.log && rm language/v1beta1/generator-errors.log
language1_beta1: language/v1beta1/lib/Cargo.toml
language1_beta1-cargo: language1_beta1
	( cd language/v1beta1/lib && cargo $(ARGS) ) 2> language/v1beta1/generator-errors.log && rm language/v1beta1/generator-errors.log

language/v1beta2/lib/Cargo.toml: language/v1beta2/spec.json $(MCP)
	$(MCP) generate $< language/v1beta2 2>language/v1beta2/generator-errors.log && rm language/v1beta2/generator-errors.log
language1_beta2: language/v1beta2/lib/Cargo.toml
language1_beta2-cargo: language1_beta2
	( cd language/v1beta2/lib && cargo $(ARGS) ) 2> language/v1beta2/generator-errors.log && rm language/v1beta2/generator-errors.log

licensing/v1/lib/Cargo.toml: licensing/v1/spec.json $(MCP)
	$(MCP) generate $< licensing/v1 2>licensing/v1/generator-errors.log && rm licensing/v1/generator-errors.log
licensing1: licensing/v1/lib/Cargo.toml
licensing1-cargo: licensing1
	( cd licensing/v1/lib && cargo $(ARGS) ) 2> licensing/v1/generator-errors.log && rm licensing/v1/generator-errors.log

pagespeedonline/v1/lib/Cargo.toml: pagespeedonline/v1/spec.json $(MCP)
	$(MCP) generate $< pagespeedonline/v1 2>pagespeedonline/v1/generator-errors.log && rm pagespeedonline/v1/generator-errors.log
pagespeedonline1: pagespeedonline/v1/lib/Cargo.toml
pagespeedonline1-cargo: pagespeedonline1
	( cd pagespeedonline/v1/lib && cargo $(ARGS) ) 2> pagespeedonline/v1/generator-errors.log && rm pagespeedonline/v1/generator-errors.log

pagespeedonline/v2/lib/Cargo.toml: pagespeedonline/v2/spec.json $(MCP)
	$(MCP) generate $< pagespeedonline/v2 2>pagespeedonline/v2/generator-errors.log && rm pagespeedonline/v2/generator-errors.log
pagespeedonline2: pagespeedonline/v2/lib/Cargo.toml
pagespeedonline2-cargo: pagespeedonline2
	( cd pagespeedonline/v2/lib && cargo $(ARGS) ) 2> pagespeedonline/v2/generator-errors.log && rm pagespeedonline/v2/generator-errors.log

pagespeedonline/v4/lib/Cargo.toml: pagespeedonline/v4/spec.json $(MCP)
	$(MCP) generate $< pagespeedonline/v4 2>pagespeedonline/v4/generator-errors.log && rm pagespeedonline/v4/generator-errors.log
pagespeedonline4: pagespeedonline/v4/lib/Cargo.toml
pagespeedonline4-cargo: pagespeedonline4
	( cd pagespeedonline/v4/lib && cargo $(ARGS) ) 2> pagespeedonline/v4/generator-errors.log && rm pagespeedonline/v4/generator-errors.log

pagespeedonline/v5/lib/Cargo.toml: pagespeedonline/v5/spec.json $(MCP)
	$(MCP) generate $< pagespeedonline/v5 2>pagespeedonline/v5/generator-errors.log && rm pagespeedonline/v5/generator-errors.log
pagespeedonline5: pagespeedonline/v5/lib/Cargo.toml
pagespeedonline5-cargo: pagespeedonline5
	( cd pagespeedonline/v5/lib && cargo $(ARGS) ) 2> pagespeedonline/v5/generator-errors.log && rm pagespeedonline/v5/generator-errors.log

plus/v1/lib/Cargo.toml: plus/v1/spec.json $(MCP)
	$(MCP) generate $< plus/v1 2>plus/v1/generator-errors.log && rm plus/v1/generator-errors.log
plus1: plus/v1/lib/Cargo.toml
plus1-cargo: plus1
	( cd plus/v1/lib && cargo $(ARGS) ) 2> plus/v1/generator-errors.log && rm plus/v1/generator-errors.log

plusDomains/v1/lib/Cargo.toml: plusDomains/v1/spec.json $(MCP)
	$(MCP) generate $< plusDomains/v1 2>plusDomains/v1/generator-errors.log && rm plusDomains/v1/generator-errors.log
plusDomains1: plusDomains/v1/lib/Cargo.toml
plusDomains1-cargo: plusDomains1
	( cd plusDomains/v1/lib && cargo $(ARGS) ) 2> plusDomains/v1/generator-errors.log && rm plusDomains/v1/generator-errors.log

pubsub/v1beta1a/lib/Cargo.toml: pubsub/v1beta1a/spec.json $(MCP)
	$(MCP) generate $< pubsub/v1beta1a 2>pubsub/v1beta1a/generator-errors.log && rm pubsub/v1beta1a/generator-errors.log
pubsub1_beta1a: pubsub/v1beta1a/lib/Cargo.toml
pubsub1_beta1a-cargo: pubsub1_beta1a
	( cd pubsub/v1beta1a/lib && cargo $(ARGS) ) 2> pubsub/v1beta1a/generator-errors.log && rm pubsub/v1beta1a/generator-errors.log

remotebuildexecution/v2/lib/Cargo.toml: remotebuildexecution/v2/spec.json $(MCP)
	$(MCP) generate $< remotebuildexecution/v2 2>remotebuildexecution/v2/generator-errors.log && rm remotebuildexecution/v2/generator-errors.log
remotebuildexecution2: remotebuildexecution/v2/lib/Cargo.toml
remotebuildexecution2-cargo: remotebuildexecution2
	( cd remotebuildexecution/v2/lib && cargo $(ARGS) ) 2> remotebuildexecution/v2/generator-errors.log && rm remotebuildexecution/v2/generator-errors.log

replicapool/v1beta1/lib/Cargo.toml: replicapool/v1beta1/spec.json $(MCP)
	$(MCP) generate $< replicapool/v1beta1 2>replicapool/v1beta1/generator-errors.log && rm replicapool/v1beta1/generator-errors.log
replicapool1_beta1: replicapool/v1beta1/lib/Cargo.toml
replicapool1_beta1-cargo: replicapool1_beta1
	( cd replicapool/v1beta1/lib && cargo $(ARGS) ) 2> replicapool/v1beta1/generator-errors.log && rm replicapool/v1beta1/generator-errors.log

reseller/v1/lib/Cargo.toml: reseller/v1/spec.json $(MCP)
	$(MCP) generate $< reseller/v1 2>reseller/v1/generator-errors.log && rm reseller/v1/generator-errors.log
reseller1: reseller/v1/lib/Cargo.toml
reseller1-cargo: reseller1
	( cd reseller/v1/lib && cargo $(ARGS) ) 2> reseller/v1/generator-errors.log && rm reseller/v1/generator-errors.log

runtimeconfig/v1/lib/Cargo.toml: runtimeconfig/v1/spec.json $(MCP)
	$(MCP) generate $< runtimeconfig/v1 2>runtimeconfig/v1/generator-errors.log && rm runtimeconfig/v1/generator-errors.log
runtimeconfig1: runtimeconfig/v1/lib/Cargo.toml
runtimeconfig1-cargo: runtimeconfig1
	( cd runtimeconfig/v1/lib && cargo $(ARGS) ) 2> runtimeconfig/v1/generator-errors.log && rm runtimeconfig/v1/generator-errors.log

servicebroker/v1/lib/Cargo.toml: servicebroker/v1/spec.json $(MCP)
	$(MCP) generate $< servicebroker/v1 2>servicebroker/v1/generator-errors.log && rm servicebroker/v1/generator-errors.log
servicebroker1: servicebroker/v1/lib/Cargo.toml
servicebroker1-cargo: servicebroker1
	( cd servicebroker/v1/lib && cargo $(ARGS) ) 2> servicebroker/v1/generator-errors.log && rm servicebroker/v1/generator-errors.log

servicecontrol/v1/lib/Cargo.toml: servicecontrol/v1/spec.json $(MCP)
	$(MCP) generate $< servicecontrol/v1 2>servicecontrol/v1/generator-errors.log && rm servicecontrol/v1/generator-errors.log
servicecontrol1: servicecontrol/v1/lib/Cargo.toml
servicecontrol1-cargo: servicecontrol1
	( cd servicecontrol/v1/lib && cargo $(ARGS) ) 2> servicecontrol/v1/generator-errors.log && rm servicecontrol/v1/generator-errors.log

siteVerification/v1/lib/Cargo.toml: siteVerification/v1/spec.json $(MCP)
	$(MCP) generate $< siteVerification/v1 2>siteVerification/v1/generator-errors.log && rm siteVerification/v1/generator-errors.log
siteVerification1: siteVerification/v1/lib/Cargo.toml
siteVerification1-cargo: siteVerification1
	( cd siteVerification/v1/lib && cargo $(ARGS) ) 2> siteVerification/v1/generator-errors.log && rm siteVerification/v1/generator-errors.log

sqladmin/v1beta4/lib/Cargo.toml: sqladmin/v1beta4/spec.json $(MCP)
	$(MCP) generate $< sqladmin/v1beta4 2>sqladmin/v1beta4/generator-errors.log && rm sqladmin/v1beta4/generator-errors.log
sqladmin1_beta4: sqladmin/v1beta4/lib/Cargo.toml
sqladmin1_beta4-cargo: sqladmin1_beta4
	( cd sqladmin/v1beta4/lib && cargo $(ARGS) ) 2> sqladmin/v1beta4/generator-errors.log && rm sqladmin/v1beta4/generator-errors.log

storage/v1beta1/lib/Cargo.toml: storage/v1beta1/spec.json $(MCP)
	$(MCP) generate $< storage/v1beta1 2>storage/v1beta1/generator-errors.log && rm storage/v1beta1/generator-errors.log
storage1_beta1: storage/v1beta1/lib/Cargo.toml
storage1_beta1-cargo: storage1_beta1
	( cd storage/v1beta1/lib && cargo $(ARGS) ) 2> storage/v1beta1/generator-errors.log && rm storage/v1beta1/generator-errors.log

storagetransfer/v1/lib/Cargo.toml: storagetransfer/v1/spec.json $(MCP)
	$(MCP) generate $< storagetransfer/v1 2>storagetransfer/v1/generator-errors.log && rm storagetransfer/v1/generator-errors.log
storagetransfer1: storagetransfer/v1/lib/Cargo.toml
storagetransfer1-cargo: storagetransfer1
	( cd storagetransfer/v1/lib && cargo $(ARGS) ) 2> storagetransfer/v1/generator-errors.log && rm storagetransfer/v1/generator-errors.log

streetviewpublish/v1/lib/Cargo.toml: streetviewpublish/v1/spec.json $(MCP)
	$(MCP) generate $< streetviewpublish/v1 2>streetviewpublish/v1/generator-errors.log && rm streetviewpublish/v1/generator-errors.log
streetviewpublish1: streetviewpublish/v1/lib/Cargo.toml
streetviewpublish1-cargo: streetviewpublish1
	( cd streetviewpublish/v1/lib && cargo $(ARGS) ) 2> streetviewpublish/v1/generator-errors.log && rm streetviewpublish/v1/generator-errors.log

surveys/v2/lib/Cargo.toml: surveys/v2/spec.json $(MCP)
	$(MCP) generate $< surveys/v2 2>surveys/v2/generator-errors.log && rm surveys/v2/generator-errors.log
surveys2: surveys/v2/lib/Cargo.toml
surveys2-cargo: surveys2
	( cd surveys/v2/lib && cargo $(ARGS) ) 2> surveys/v2/generator-errors.log && rm surveys/v2/generator-errors.log

texttospeech/v1/lib/Cargo.toml: texttospeech/v1/spec.json $(MCP)
	$(MCP) generate $< texttospeech/v1 2>texttospeech/v1/generator-errors.log && rm texttospeech/v1/generator-errors.log
texttospeech1: texttospeech/v1/lib/Cargo.toml
texttospeech1-cargo: texttospeech1
	( cd texttospeech/v1/lib && cargo $(ARGS) ) 2> texttospeech/v1/generator-errors.log && rm texttospeech/v1/generator-errors.log

texttospeech/v1beta1/lib/Cargo.toml: texttospeech/v1beta1/spec.json $(MCP)
	$(MCP) generate $< texttospeech/v1beta1 2>texttospeech/v1beta1/generator-errors.log && rm texttospeech/v1beta1/generator-errors.log
texttospeech1_beta1: texttospeech/v1beta1/lib/Cargo.toml
texttospeech1_beta1-cargo: texttospeech1_beta1
	( cd texttospeech/v1beta1/lib && cargo $(ARGS) ) 2> texttospeech/v1beta1/generator-errors.log && rm texttospeech/v1beta1/generator-errors.log

urlshortener/v1/lib/Cargo.toml: urlshortener/v1/spec.json $(MCP)
	$(MCP) generate $< urlshortener/v1 2>urlshortener/v1/generator-errors.log && rm urlshortener/v1/generator-errors.log
urlshortener1: urlshortener/v1/lib/Cargo.toml
urlshortener1-cargo: urlshortener1
	( cd urlshortener/v1/lib && cargo $(ARGS) ) 2> urlshortener/v1/generator-errors.log && rm urlshortener/v1/generator-errors.log

videointelligence/v1p1beta1/lib/Cargo.toml: videointelligence/v1p1beta1/spec.json $(MCP)
	$(MCP) generate $< videointelligence/v1p1beta1 2>videointelligence/v1p1beta1/generator-errors.log && rm videointelligence/v1p1beta1/generator-errors.log
videointelligence1_p1beta1: videointelligence/v1p1beta1/lib/Cargo.toml
videointelligence1_p1beta1-cargo: videointelligence1_p1beta1
	( cd videointelligence/v1p1beta1/lib && cargo $(ARGS) ) 2> videointelligence/v1p1beta1/generator-errors.log && rm videointelligence/v1p1beta1/generator-errors.log

videointelligence/v1p2beta1/lib/Cargo.toml: videointelligence/v1p2beta1/spec.json $(MCP)
	$(MCP) generate $< videointelligence/v1p2beta1 2>videointelligence/v1p2beta1/generator-errors.log && rm videointelligence/v1p2beta1/generator-errors.log
videointelligence1_p2beta1: videointelligence/v1p2beta1/lib/Cargo.toml
videointelligence1_p2beta1-cargo: videointelligence1_p2beta1
	( cd videointelligence/v1p2beta1/lib && cargo $(ARGS) ) 2> videointelligence/v1p2beta1/generator-errors.log && rm videointelligence/v1p2beta1/generator-errors.log

videointelligence/v1p3beta1/lib/Cargo.toml: videointelligence/v1p3beta1/spec.json $(MCP)
	$(MCP) generate $< videointelligence/v1p3beta1 2>videointelligence/v1p3beta1/generator-errors.log && rm videointelligence/v1p3beta1/generator-errors.log
videointelligence1_p3beta1: videointelligence/v1p3beta1/lib/Cargo.toml
videointelligence1_p3beta1-cargo: videointelligence1_p3beta1
	( cd videointelligence/v1p3beta1/lib && cargo $(ARGS) ) 2> videointelligence/v1p3beta1/generator-errors.log && rm videointelligence/v1p3beta1/generator-errors.log

videointelligence/v1beta2/lib/Cargo.toml: videointelligence/v1beta2/spec.json $(MCP)
	$(MCP) generate $< videointelligence/v1beta2 2>videointelligence/v1beta2/generator-errors.log && rm videointelligence/v1beta2/generator-errors.log
videointelligence1_beta2: videointelligence/v1beta2/lib/Cargo.toml
videointelligence1_beta2-cargo: videointelligence1_beta2
	( cd videointelligence/v1beta2/lib && cargo $(ARGS) ) 2> videointelligence/v1beta2/generator-errors.log && rm videointelligence/v1beta2/generator-errors.log

webmasters/v3/lib/Cargo.toml: webmasters/v3/spec.json $(MCP)
	$(MCP) generate $< webmasters/v3 2>webmasters/v3/generator-errors.log && rm webmasters/v3/generator-errors.log
webmasters3: webmasters/v3/lib/Cargo.toml
webmasters3-cargo: webmasters3
	( cd webmasters/v3/lib && cargo $(ARGS) ) 2> webmasters/v3/generator-errors.log && rm webmasters/v3/generator-errors.log

youtubeAnalytics/v1/lib/Cargo.toml: youtubeAnalytics/v1/spec.json $(MCP)
	$(MCP) generate $< youtubeAnalytics/v1 2>youtubeAnalytics/v1/generator-errors.log && rm youtubeAnalytics/v1/generator-errors.log
youtubeAnalytics1: youtubeAnalytics/v1/lib/Cargo.toml
youtubeAnalytics1-cargo: youtubeAnalytics1
	( cd youtubeAnalytics/v1/lib && cargo $(ARGS) ) 2> youtubeAnalytics/v1/generator-errors.log && rm youtubeAnalytics/v1/generator-errors.log

youtubeAnalytics/v2/lib/Cargo.toml: youtubeAnalytics/v2/spec.json $(MCP)
	$(MCP) generate $< youtubeAnalytics/v2 2>youtubeAnalytics/v2/generator-errors.log && rm youtubeAnalytics/v2/generator-errors.log
youtubeAnalytics2: youtubeAnalytics/v2/lib/Cargo.toml
youtubeAnalytics2-cargo: youtubeAnalytics2
	( cd youtubeAnalytics/v2/lib && cargo $(ARGS) ) 2> youtubeAnalytics/v2/generator-errors.log && rm youtubeAnalytics/v2/generator-errors.log
