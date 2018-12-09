#[macro_use]
extern crate derive_is_enum_variant;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate unhtml_derive;

extern crate unhtml;

use unhtml::FromHtml;
use failure::Error;

#[derive(Debug, Clone)]
#[derive(is_enum_variant)]
pub enum Event {
    ProcessCreate(ProcessCreateEvent),
    FileCreate(FileCreateEvent),
    InboundNetwork(NetworkEvent),
    OutboundNetwork(NetworkEvent),
}

#[derive(Debug, FromHtml, Clone)]
struct Type {
    #[html(selector = "EventID", attr = "inner")]
    _type: u8
}

#[derive(Debug, FromHtml, Clone)]
struct NetworkType {
    #[html(selector = "EventData > Data[Name=\"Initiated\"]", attr="inner")]
    _type: bool
}

impl Event {
    pub fn from_str(s: impl AsRef<str>) -> Result<Self, Error> {
        let event_type = Type::from_html(s.as_ref())?._type;

        match event_type {
            1 => {
                let event = ProcessCreateEvent::from_html(s.as_ref())?;
                Ok(Event::ProcessCreate(event))
            },
            11 => {
                let event = FileCreateEvent::from_html(s.as_ref())?;
                Ok(Event::FileCreate(event))
            },
            3 => {
                let event = NetworkEvent::from_html(s.as_ref())?;
                if NetworkType::from_html(s.as_ref())?._type {
                    Ok(Event::OutboundNetwork(event))
                } else {
                    Ok(Event::InboundNetwork(event))
                }
            },
            _ => bail!("Unsupported event type! {}", event_type)
        }

    }
}

#[derive(Debug, FromHtml, Clone)]
pub struct SysmonSystemHeader {
    #[html(selector = "System > Provider", attr = "name")]
    pub provider_name: String,
    #[html(selector = "System > Provider", attr = "guid")]
    pub provider_guid: String,
    #[html(selector = "EventID", attr = "inner")]
    pub event_id: u8,
    #[html(selector = "Version", attr = "inner")]
    pub version: u8,
    #[html(selector = "Level", attr = "inner")]
    pub level: u8,
    #[html(selector = "Task", attr = "inner")]
    pub task: u8,
    #[html(selector = "Opcode", attr = "inner")]
    pub opcode: u8,
    #[html(selector = "Keywords", attr = "inner")]
    pub keywords: String,
    #[html(selector = "TimeCreated", attr="systemtime")]
    pub time_created: String,
    #[html(selector = "EventRecordID", attr = "inner")]
    pub event_record_id: u32,
    #[html(selector = "Channel", attr = "inner")]
    pub channel: String,
    #[html(selector = "Computer", attr = "inner")]
    pub computer: String,
    #[html(selector = "Security", attr="userid")]
    pub sid: String
}

#[derive(Debug, FromHtml, Clone)]
pub struct ProcessCreateEvent {
    #[html(selector = "System")]
    pub header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    pub utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    pub process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    pub process_id: u32,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    pub image: String,

    #[html(selector = "EventData > Data[Name=\"CommandLine\"]", attr="inner")]
    pub command_line: String,

    #[html(selector = "EventData > Data[Name=\"CurrentDirectory\"]", attr="inner")]
    pub current_directory: String,

    #[html(selector = "EventData > Data[Name=\"User\"]", attr="inner")]
    pub user: String,

    #[html(selector = "EventData > Data[Name=\"LogonGuid\"]", attr="inner")]
    pub logon_guid: String,

    #[html(selector = "EventData > Data[Name=\"LogonId\"]", attr="inner")]
    pub logon_id: String,

    #[html(selector = "EventData > Data[Name=\"TerminalSessionId\"]", attr="inner")]
    pub terminal_session_id: u16,

    #[html(selector = "EventData > Data[Name=\"IntegrityLevel\"]", attr="inner")]
    pub integrity_level: String,

    #[html(selector = "EventData > Data[Name=\"Hashes\"]", attr="inner")]
    pub hashes: String,

    #[html(selector = "EventData > Data[Name=\"ParentProcessGuid\"]", attr="inner")]
    pub parent_process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ParentProcessId\"]", attr="inner")]
    pub parent_process_id: u64,

    #[html(selector = "EventData > Data[Name=\"ParentImage\"]", attr="inner")]
    pub parent_image: String,

    #[html(selector = "[Name=\"ParentCommandLine\"]", attr="inner")]
    pub parent_command_line: String,
}

#[derive(Debug, FromHtml, Clone)]
pub struct FileCreateEvent {
    #[html(selector = "System")]
    pub header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    pub utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    pub process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    pub process_id: u64,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    pub image: String,

    #[html(selector = "EventData > Data[Name=\"TargetFilename\"]", attr="inner")]
    pub target_filename: String,

    #[html(selector = "EventData > Data[Name=\"CreationUtcTime\"]", attr="inner")]
    pub creation_utc_time: String,
}


#[derive(Debug, FromHtml, Clone)]
pub struct NetworkEvent {
    #[html(selector = "System")]
    pub header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    pub utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    pub process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    pub process_id: u64,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    pub image: String,

    #[html(selector = "EventData > Data[Name=\"User\"]", attr="inner")]
    pub user: String,

    #[html(selector = "EventData > Data[Name=\"Protocol\"]", attr="inner")]
    pub protocol: String,

    #[html(selector = "EventData > Data[Name=\"Initiated\"]", attr="inner")]
    pub initiated: bool,

    #[html(selector = "EventData > Data[Name=\"SourceIsIpv6\"]", attr="inner")]
    pub source_is_ipv6: String,

    #[html(selector = "EventData > Data[Name=\"SourceIp\"]", attr="inner")]
    pub source_ip: String,

    #[html(selector = "EventData > Data[Name=\"SourceHostname\"]", attr="inner")]
    pub source_hostname: String,

    #[html(selector = "EventData > Data[Name=\"SourcePort\"]", attr="inner")]
    pub source_port: u16,

    #[html(selector = "EventData > Data[Name=\"SourcePortName\"]", attr="inner")]
    pub source_port_name: String,

    #[html(selector = "EventData > Data[Name=\"DestinationIsIpv6\"]", attr="inner")]
    pub destination_is_ipv6: String,

    #[html(selector = "EventData > Data[Name=\"DestinationIp\"]", attr="inner")]
    pub destination_ip: String,

    #[html(selector = "EventData > Data[Name=\"DestinationHostname\"]", attr="inner")]
    pub destination_hostname: String,

    #[html(selector = "EventData > Data[Name=\"DestinationPort\"]", attr="inner")]
    pub destination_port: u16,

    #[html(selector = "EventData > Data[Name=\"DestinationPortName\"]", attr="inner")]
    pub destination_port_name: String,
}


#[cfg(test)]
mod tests {

    use super::*;

    const NETWORK_EVENT: &str = r#"
    <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
        <System>
            <Provider Name="Microsoft-Windows-Sysmon" Guid="{5770385F-C22A-43E0-BF4C-06F5698FFBD9}" />
            <EventID>3</EventID>
            <Version>5</Version>
            <Level>4</Level>
            <Task>3</Task>
            <Opcode>0</Opcode>
            <Keywords>0x8000000000000000</Keywords>
            <TimeCreated SystemTime="2017-04-28T22:12:23.657698300Z" />
            <EventRecordID>10953</EventRecordID>
            <Correlation />
            <Execution ProcessID="3216" ThreadID="3976" />
            <Channel>Microsoft-Windows-Sysmon/Operational</Channel>
            <Computer>rfsH.lab.local</Computer>
            <Security UserID="S-1-5-18" />
        </System>
        <EventData>
            <Data Name="UtcTime">2017-04-28 22:12:22.557</Data>
            <Data Name="ProcessGuid">{A23EAE89-BD28-5903-0000-00102F345D00}</Data>
            <Data Name="ProcessId">13220</Data>
            <Data Name="Image">C:\Program Files (x86)\Google\Chrome\Application\chrome.exe</Data>
            <Data Name="User">LAB\rsmith</Data>
            <Data Name="Protocol">tcp</Data>
            <Data Name="Initiated">true</Data>
            <Data Name="SourceIsIpv6">false</Data>
            <Data Name="SourceIp">192.168.1.250</Data>
            <Data Name="SourceHostname">rfsH.lab.local</Data>
            <Data Name="SourcePort">3328</Data>
            <Data Name="SourcePortName"></Data>
            <Data Name="DestinationIsIpv6">false</Data>
            <Data Name="DestinationIp">104.130.229.150</Data>
            <Data Name="DestinationHostname"></Data>
            <Data Name="DestinationPort">443</Data>
            <Data Name="DestinationPortName">https</Data>
        </EventData>
    </Event>
    "#;

    const FILE_CREATE: &str = r#"
        <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
        <System>
            <Provider Name="Microsoft-Windows-Sysmon" Guid="{5770385F-C22A-43E0-BF4C-06F5698FFBD9}" />
            <EventID>11</EventID>
            <Version>2</Version>
            <Level>4</Level>
            <Task>11</Task>
            <Opcode>0</Opcode>
            <Keywords>0x8000000000000000</Keywords>
            <TimeCreated SystemTime="2017-05-13T19:44:55.314125100Z" />
            <EventRecordID>734181</EventRecordID>
            <Correlation />
            <Execution ProcessID="2848" ThreadID="3520" />
            <Channel>Microsoft-Windows-Sysmon/Operational</Channel>
            <Computer>rfsH.lab.local</Computer>
            <Security UserID="S-1-5-18" />
        </System>
        <EventData>
            <Data Name="UtcTime">2017-05-13 19:44:55.313</Data>
            <Data Name="ProcessGuid">{A23EAE89-6237-5917-0000-0010300E6601}</Data>
            <Data Name="ProcessId">19200</Data>
            <Data Name="Image">C:\Windows\Microsoft.NET\Framework64\v4.0.30319\mscorsvw.exe</Data>
            <Data Name="TargetFilename">C:\Windows\assembly\NativeImages_v4.0.30319_64\Temp\4b00-0\AxImp.exe</Data>
            <Data Name="CreationUtcTime">2017-05-13 19:44:55.313</Data>
        </EventData>
        </Event>
    "#;

    const PROCESS_CREATE: &str = r#"
    <Event xmlns="http://schemas.microsoft.com/win/2004/08/events/event">
        <System>
            <Provider Name="Microsoft-Windows-Sysmon" Guid="{5770385F-C22A-43E0-BF4C-06F5698FFBD9}" />
            <EventID>1</EventID>
            <Version>5</Version>
            <Level>4</Level>
            <Task>1</Task>
            <Opcode>0</Opcode>
            <Keywords>0x8000000000000000</Keywords>
            <TimeCreated SystemTime="2017-04-28T22:08:22.025812200Z" />
            <EventRecordID>9947</EventRecordID>
            <Correlation />
            <Execution ProcessID="3216" ThreadID="3964" />
            <Channel>Microsoft-Windows-Sysmon/Operational</Channel>
            <Computer>rfsH.lab.local</Computer>
            <Security UserID="S-1-5-18" />
        </System>
        <EventData>
            <Data Name="UtcTime">2017-04-28 22:08:22.025</Data>
            <Data Name="ProcessGuid">{A23EAE89-BD56-5903-0000-0010E9D95E00}</Data>
            <Data Name="ProcessId">6228</Data>
            <Data Name="Image">C:\Program Files (x86)\Google\Chrome\Application\chrome.exe</Data>
            <Data Name="CommandLine">"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" --type=utility --lang=en-US --no-sandbox --service-request-channel-token=F47498BBA884E523FA93E623C4569B94 --mojo-platform-channel-handle=3432 /prefetch:8</Data>
            <Data Name="CurrentDirectory">C:\Program Files (x86)\Google\Chrome\Application\58.0.3029.81\</Data>
            <Data Name="User">LAB\rsmith</Data>
            <Data Name="LogonGuid">{A23EAE89-B357-5903-0000-002005EB0700}</Data>
            <Data Name="LogonId">0x7eb05</Data>
            <Data Name="TerminalSessionId">1</Data>
            <Data Name="IntegrityLevel">Medium</Data>
            <Data Name="Hashes">SHA256=6055A20CF7EC81843310AD37700FF67B2CF8CDE3DCE68D54BA42934177C10B57</Data>
            <Data Name="ParentProcessGuid">{A23EAE89-BD28-5903-0000-00102F345D00}</Data>
            <Data Name="ParentProcessId">13220</Data>
            <Data Name="ParentImage">C:\Program Files (x86)\Google\Chrome\Application\chrome.exe</Data>
            <Data Name="ParentCommandLine">"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe" </Data>
        </EventData>
    </Event>
    "#;

    #[test]
    fn process_create_event() {
        ProcessCreateEvent::from_html(PROCESS_CREATE).unwrap();
    }

    #[test]
    fn file_create_event() {
        FileCreateEvent::from_html(FILE_CREATE).unwrap();
    }

    #[test]
    fn network_event() {
        NetworkEvent::from_html(NETWORK_EVENT).unwrap();
    }

    #[test]
    fn event_type() {
        assert!(Event::from_str(NETWORK_EVENT).unwrap().is_outbound_network());
        assert!(Event::from_str(FILE_CREATE).unwrap().is_file_create());
        assert!(Event::from_str(PROCESS_CREATE).unwrap().is_process_create());
    }
}


