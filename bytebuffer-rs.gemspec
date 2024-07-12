GEM_SPEC = Gem::Specification.new do |spec|
  spec.name = 'bytebuffer-rs'
  spec.version = ENV['GEM_VERSION'] || '0.0.0'
  spec.summary = 'A gem wrapping the bytebuffer crate using FFI.'
  spec.description = 'A gem that provides a ByteBuffer class based on the bytebuffer rust crate.'
  spec.license = 'BSD-3-Clause'
  spec.author = 'Patrick W.'
  spec.email = 'Sickday@pm.me'

  spec.required_ruby_version = '>= 3.1.0'

  spec.add_runtime_dependency 'ffi', '~> 1.17'

  spec.add_development_dependency 'benchmark', '~> 0.3'
  spec.add_development_dependency 'rake', '~> 13.0'
  spec.add_development_dependency 'rspec', '~> 3.12'
  spec.add_development_dependency 'minitest', '~> 5.19'
  spec.add_development_dependency 'yard', '~> 0.9'

  spec.extensions = "ext/extconf.rb"
  spec.require_paths = ['lib']

  spec.files = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
end
