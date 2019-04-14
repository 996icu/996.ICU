Gem::Specification.new do |s|
  s.name                   = 'gen-license'
  s.version                = '0.0.1'
  s.date                   = '2019-04-14'
  s.summary                = 'Generate LICENSE Files'
  s.description            = 'Generate LICENSE Files'
  s.author                 = 'Dongri Jin'
  s.email                  = 'dongrify@gmail.com'
  s.homepage               = 'https://github.com/dongri/gen-license'
  s.files                  = `git ls-files`.split($\)
  s.executables            = s.files.grep(%r{^bin}) { |f| File.basename(f) }
end
