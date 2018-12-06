#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;
use unhtml::*;

#[derive(Debug, FromHtml)]
struct SysmonSystemHeader {
    #[html(selector = "System > Provider", attr = "name")]
    provider_name: String,
    #[html(selector = "System > Provider", attr = "guid")]
    provider_guid: String,
    #[html(selector = "EventID", attr = "inner")]
    event_id: u8,
    #[html(selector = "Version", attr = "inner")]
    version: u8,
    #[html(selector = "Level", attr = "inner")]
    level: u8,
    #[html(selector = "Task", attr = "inner")]
    task: u8,
    #[html(selector = "Opcode", attr = "inner")]
    opcode: u8,
    #[html(selector = "Keywords", attr = "inner")]
    keywords: String,
    #[html(selector = "TimeCreated", attr="systemtime")]
    time_created: String,
    #[html(selector = "EventRecordID", attr = "inner")]
    event_record_id: u32,
    #[html(selector = "Channel", attr = "inner")]
    channel: String,
    #[html(selector = "Computer", attr = "inner")]
    computer: String,
    #[html(selector = "Security", attr="userid")]
    sid: String
}

#[derive(Debug, FromHtml)]
struct ProcessCreateEvent {
    #[html(selector = "System")]
    header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    process_id: u32,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    image: String,

    #[html(selector = "EventData > Data[Name=\"CommandLine\"]", attr="inner")]
    command_line: String,

    #[html(selector = "EventData > Data[Name=\"CurrentDirectory\"]", attr="inner")]
    current_directory: String,

    #[html(selector = "EventData > Data[Name=\"User\"]", attr="inner")]
    user: String,

    #[html(selector = "EventData > Data[Name=\"LogonGuid\"]", attr="inner")]
    logon_guid: String,

    #[html(selector = "EventData > Data[Name=\"LogonId\"]", attr="inner")]
    logon_id: String,

    #[html(selector = "EventData > Data[Name=\"TerminalSessionId\"]", attr="inner")]
    terminal_session_id: u16,

    #[html(selector = "EventData > Data[Name=\"IntegrityLevel\"]", attr="inner")]
    integrity_level: String,

    #[html(selector = "EventData > Data[Name=\"Hashes\"]", attr="inner")]
    hashes: String,

    #[html(selector = "EventData > Data[Name=\"ParentProcessGuid\"]", attr="inner")]
    parent_process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ParentProcessId\"]", attr="inner")]
    parent_process_id: String,

    #[html(selector = "EventData > Data[Name=\"ParentImage\"]", attr="inner")]
    parent_image: String,

    #[html(selector = "[Name=\"ParentCommandLine\"]", attr="inner")]
    parent_command_line: String,
}


#[derive(Debug, FromHtml)]
struct FileCreateEvent {
    #[html(selector = "System")]
    header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    process_id: String,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    image: String,

    #[html(selector = "EventData > Data[Name=\"TargetFilename\"]", attr="inner")]
    target_filename: String,

    #[html(selector = "EventData > Data[Name=\"CreationUtcTime\"]", attr="inner")]
    creation_utc_time: String,
}


#[derive(Debug, FromHtml)]
struct NetworkEvent {
    #[html(selector = "System")]
    header: SysmonSystemHeader,

    #[html(selector = "EventData > Data[Name=\"UtcTime\"]", attr="inner")]
    utc_time: String,

    #[html(selector = "EventData > Data[Name=\"ProcessGuid\"]", attr="inner")]
    process_guid: String,

    #[html(selector = "EventData > Data[Name=\"ProcessId\"]", attr="inner")]
    process_id: String,

    #[html(selector = "EventData > Data[Name=\"Image\"]", attr="inner")]
    image: String,

    #[html(selector = "EventData > Data[Name=\"User\"]", attr="inner")]
    user: String,

    #[html(selector = "EventData > Data[Name=\"Protocol\"]", attr="inner")]
    protocol: String,

    #[html(selector = "EventData > Data[Name=\"Initiated\"]", attr="inner")]
    initiated: String,

    #[html(selector = "EventData > Data[Name=\"SourceIsIpv6\"]", attr="inner")]
    source_is_ipv6: String,

    #[html(selector = "EventData > Data[Name=\"SourceIp\"]", attr="inner")]
    source_ip: String,

    #[html(selector = "EventData > Data[Name=\"SourceHostname\"]", attr="inner")]
    source_hostname: String,

    #[html(selector = "EventData > Data[Name=\"SourcePort\"]", attr="inner")]
    source_port: String,

    #[html(selector = "EventData > Data[Name=\"SourcePortName\"]", attr="inner")]
    source_port_name: String,

    #[html(selector = "EventData > Data[Name=\"DestinationIsIpv6\"]", attr="inner")]
    destination_is_ipv6: String,

    #[html(selector = "EventData > Data[Name=\"DestinationIp\"]", attr="inner")]
    destination_ip: String,

    #[html(selector = "EventData > Data[Name=\"DestinationHostname\"]", attr="inner")]
    destination_hostname: String,

    #[html(selector = "EventData > Data[Name=\"DestinationPort\"]", attr="inner")]
    destination_port: String,

    #[html(selector = "EventData > Data[Name=\"DestinationPortName\"]", attr="inner")]
    destination_port_name: String,
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
}


