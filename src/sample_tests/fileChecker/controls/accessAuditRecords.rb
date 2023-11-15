# copyright: 2023, nmcspadden

title 'Control for MacOS CIS Benchmarks Section 3'

control 'Informational Test: accessAuditRecords' do
  impact 0.0
  title '3.5 Control Access to audit records'
  desc 'Ensure Access to audit records is correctly set'
  ref 'AccessAuditRecordsCompliant'

  files = [
      '/etc/security/audit_control',
      '/var/audit/current',
  ]

  files.each do |file_path|
    describe file(file_path) do
        it { should exist }
        its('owner') { should eq 'root' }
        its('group') { should be_in ['root', 'wheel'] }
        it { should_not be_more_permissive_than('0440') }
    end
  end
end
